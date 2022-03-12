use eframe::{
    egui::{CollapsingHeader, RichText, Ui},
    epaint::Color32,
};

#[derive(Clone, Copy, PartialEq)]
pub enum Action {
    Keep,
    Delete,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Tree(String, SubTree);

impl Tree {
    pub fn none() -> Self {
        Self(
            String::from("root"),
            SubTree(vec![
                SubTree(vec![SubTree::default(); 4]),
                SubTree(vec![SubTree(vec![SubTree::default(); 2]); 3]),
            ]),
        )
    }
    pub fn ui(&mut self, ui: &mut Ui) -> Action {
        self.1.ui(ui, 0, "root", &mut self.0)
    }
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct SubTree(Vec<SubTree>);

impl SubTree {
    pub fn ui(
        &mut self,
        ui: &mut Ui,
        depth: usize,
        name: &str,
        selected_name: &mut String,
    ) -> Action {
        let response = CollapsingHeader::new(name)
            .default_open(depth < 1)
            .selectable(true)
            .selected(selected_name.as_str() == name)
            .show(ui, |ui| self.children_ui(ui, name, depth, selected_name));
        if response.header_response.clicked() {
            *selected_name = name.to_string();
        }
        response.body_returned.unwrap_or(Action::Keep)
    }

    fn children_ui(
        &mut self,
        ui: &mut Ui,
        parent_name: &str,
        depth: usize,
        selected_name: &mut String,
    ) -> Action {
        if depth > 0 {
            for i in 0..=10 {
                let bt = ui.button(RichText::new(&format!("D - {}", i)).color(Color32::GREEN));
                if bt.hovered() {
                    // println!("hovered {}", i);
                }
                if bt.clicked() {
                    // println!("delete {}", i);
                }
            }
        }
        if depth > 0
            && ui
                .button(RichText::new("delete").color(Color32::RED))
                .clicked()
        {
            return Action::Delete;
        }

        self.0 = std::mem::take(self)
            .0
            .into_iter()
            .enumerate()
            .filter_map(|(i, mut tree)| {
                if tree.ui(
                    ui,
                    depth + 1,
                    &format!("{}/{}", parent_name, i),
                    selected_name,
                ) == Action::Keep
                {
                    // 可以重新组织UI
                    Some(tree)
                } else {
                    None
                }
            })
            .collect();

        if ui.button("+").clicked() {
            self.0.push(SubTree::default());
        }

        Action::Keep
    }
}
