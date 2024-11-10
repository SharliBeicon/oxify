extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(OxifyTable, attributes(skip))]
pub fn oxify_tab_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match input.data {
        syn::Data::Struct(data) => data.fields,
        _ => panic!("OxifyTable can only be used with structs"),
    };

    let table_name = proc_macro2::Ident::new(
        &format!("{}{}", quote! {#name}.to_string(), "Table"),
        proc_macro2::Span::call_site(),
    );

    let field_names: Vec<String> = fields
        .iter()
        .filter_map(|field| {
            let skip_attr = field.attrs.iter().any(|attr| attr.path().is_ident("skip"));
            if !skip_attr {
                Some(field.ident.as_ref().unwrap().to_string())
            } else {
                None
            }
        })
        .collect();

    let field_refs = fields.iter().map(|field| &field.ident);
    let array_size = fields.len();

    let expanded = quote! {
        impl #name {
            pub const fn ref_array(&self) -> [&String; #array_size] {
                [#(&self.#field_refs),*]
            }
        }

        #[derive(Debug, Clone)]
        pub struct #table_name {
            state: State,
            items: Vec<#name>,
        }

        impl #table_name {
            pub fn field_names() -> Vec<String> {
                vec![#(#field_names.to_string()),*]
            }

            pub fn new<T: Into<Vec<#name>>>(collection: T) -> Self
                where
                    Vec<#name>: From<T>
            {
                let items: Vec<#name> = collection.into();

                Self {
                    state: State {
                        table_state: TableState::default().with_selected(0),
                        scroll_state: ScrollbarState::new((items.len() - 1) * ITEM_HEIGHT),
                        colors: TableColors::new(&tailwind::AMBER),
                    },
                    items,
                }
            }

            pub fn next_row(&mut self) {
                let i = match self.state.table_state.selected() {
                    Some(i) => {
                        if i >= self.items.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.state.table_state.select(Some(i));
                self.state.scroll_state = self.state.scroll_state.position(i * ITEM_HEIGHT);
            }

            pub fn selected_uri(&mut self) -> Option<String> {
                let selected = self.state.table_state.selected()?;

                Some(self.items[selected].uri.clone())
            }

            pub fn previous_row(&mut self) {
                let i = match self.state.table_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.items.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.state.table_state.select(Some(i));
                self.state.scroll_state = self.state.scroll_state.position(i * ITEM_HEIGHT);
            }

            pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
                self.render_table(frame, area);
                self.render_scrollbar(frame, area);
            }

            fn render_table(&mut self, frame: &mut Frame, area: Rect) {
                let header_style = Style::default()
                    .fg(self.state.colors.header_fg)
                    .bg(self.state.colors.header_bg);
                let selected_row_style = Style::default()
                    .add_modifier(Modifier::REVERSED)
                    .fg(self.state.colors.selected_row_style_fg);

                let header = #table_name::field_names()
                    .into_iter()
                    .map(Cell::from)
                    .collect::<Row>()
                    .style(header_style)
                    .height(1);
                let rows = self.items.iter().map(|data| {
                    let item = data.ref_array();
                    item.into_iter()
                        .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                        .collect::<Row>()
                        .style(Style::default())
                        .height(2)
                });
                let bar = " █ ";
                let t = Table::new(
                    rows,
                    [
                        Constraint::Percentage(30),
                        Constraint::Percentage(30),
                        Constraint::Percentage(25),
                        Constraint::Percentage(15),
                    ],
                )
                .header(header)
                .highlight_style(selected_row_style)
                .highlight_symbol(Text::from(vec![
                    "".into(),
                    bar.into(),
                    bar.into(),
                    "".into(),
                ]))
                .highlight_spacing(HighlightSpacing::Always);
                frame.render_stateful_widget(t, area, &mut self.state.table_state);
            }

            fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
                frame.render_stateful_widget(
                    Scrollbar::default()
                        .orientation(ScrollbarOrientation::VerticalRight)
                        .begin_symbol(None)
                        .end_symbol(None),
                    area.inner(Margin {
                        vertical: 1,
                        horizontal: 1,
                    }),
                    &mut self.state.scroll_state,
                );
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
