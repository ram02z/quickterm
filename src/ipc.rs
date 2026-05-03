use swayipc::{Connection, Node, NodeType, Rect};

use crate::config::Position;
use crate::error::QuicktermError;

pub const MARK_QT_PATTERN: &str = "quickterm_.*";

pub fn mark_for_shell(shell: &str) -> String {
    format!("quickterm_{shell}")
}

pub struct WorkspaceInfo {
    pub name: String,
    pub rect: Rect,
}

pub struct Ipc {
    connection: Connection,
}

fn popup_geometry(rect: Rect, pos: &Position, ratio: f64) -> (i32, i32, i32, i32) {
    let wx = rect.x;
    let wy = rect.y;
    let mut width = rect.width;
    let wheight = rect.height;

    let height = (wheight as f64 * ratio) as i32;
    let (posx, posy) = match pos {
        Position::Bottom => (wx, wy + wheight - height - 6),
        Position::Center => {
            width = (width as f64 * (ratio / 1.25)) as i32;
            let posx = wx + (rect.width - width) / 2;
            let posy = wy + (wheight - height) / 2;
            (posx, posy)
        }
        Position::Top => (wx, wy),
    };

    (width, height, posx, posy)
}

impl Ipc {
    pub fn connect() -> Result<Self, QuicktermError> {
        let connection = Connection::new().map_err(|err| QuicktermError::Ipc(err.to_string()))?;
        Ok(Self { connection })
    }

    pub fn current_workspace(&mut self) -> Result<WorkspaceInfo, QuicktermError> {
        let workspace = self
            .connection
            .get_workspaces()
            .map_err(|err| QuicktermError::Ipc(err.to_string()))?
            .into_iter()
            .find(|workspace| workspace.focused)
            .ok_or_else(|| QuicktermError::Ipc("no current workspace".to_string()))?;

        Ok(WorkspaceInfo {
            name: workspace.name,
            rect: workspace.rect,
        })
    }

    pub fn command(&mut self, command: &str) -> Result<(), QuicktermError> {
        self.connection
            .run_command(command)
            .map_err(|err| QuicktermError::Ipc(err.to_string()))?;
        Ok(())
    }

    pub fn find_marked_exact(&mut self, mark: &str) -> Result<Vec<Node>, QuicktermError> {
        let tree = self
            .connection
            .get_tree()
            .map_err(|err| QuicktermError::Ipc(err.to_string()))?;
        Ok(find_marked(&tree, mark))
    }

    pub fn find_marked_pattern_in_current_workspace(
        &mut self,
        pattern: &str,
    ) -> Result<Vec<Node>, QuicktermError> {
        let workspace = self.current_workspace_node()?;
        Ok(find_marked(&workspace, pattern))
    }

    pub fn workspace_name_for_node(
        &mut self,
        node_id: i64,
    ) -> Result<Option<String>, QuicktermError> {
        let tree = self
            .connection
            .get_tree()
            .map_err(|err| QuicktermError::Ipc(err.to_string()))?;
        Ok(find_workspace_name_for_node(&tree, node_id, None))
    }

    pub fn find_titled_in_current_workspace(
        &mut self,
        title: &str,
    ) -> Result<Vec<Node>, QuicktermError> {
        let workspace = self.current_workspace_node()?;
        Ok(workspace
            .iter()
            .filter(|node| node.name.as_deref() == Some(title))
            .cloned()
            .collect())
    }

    fn current_workspace_node(&mut self) -> Result<Node, QuicktermError> {
        let workspace_name = self.current_workspace()?.name;
        let tree = self
            .connection
            .get_tree()
            .map_err(|err| QuicktermError::Ipc(err.to_string()))?;
        tree.find_as_ref(|node| {
            node.node_type == NodeType::Workspace && node.name.as_deref() == Some(&workspace_name)
        })
        .cloned()
        .ok_or_else(|| QuicktermError::Ipc("no current workspace".to_string()))
    }

    pub fn move_back(&mut self, selector: &str) -> Result<(), QuicktermError> {
        self.command(&format!("{selector} floating enable, move scratchpad"))
    }

    pub fn pop_it(&mut self, mark: &str, pos: &Position, ratio: f64) -> Result<(), QuicktermError> {
        let ws = self.current_workspace()?;
        let (width, height, posx, posy) = popup_geometry(ws.rect, pos, ratio);

        self.command(&format!(
            "[con_mark={mark}], move scratchpad, scratchpad show, resize set {width} px {height} px, move absolute position {posx}px {posy}px"
        ))
    }
}

fn find_marked(node: &Node, pattern: &str) -> Vec<Node> {
    node.iter()
        .filter(|candidate| {
            candidate
                .marks
                .iter()
                .any(|mark| mark_matches(mark, pattern))
        })
        .cloned()
        .collect()
}

fn mark_matches(mark: &str, pattern: &str) -> bool {
    if let Some(prefix) = pattern.strip_suffix(".*") {
        mark.starts_with(prefix)
    } else {
        mark == pattern
    }
}

fn find_workspace_name_for_node(
    node: &Node,
    target_id: i64,
    workspace_name: Option<&str>,
) -> Option<String> {
    let workspace_name = if node.node_type == NodeType::Workspace {
        node.name.as_deref()
    } else {
        workspace_name
    };

    if node.id == target_id {
        return workspace_name.map(str::to_string);
    }

    for child in node.nodes.iter().chain(node.floating_nodes.iter()) {
        if let Some(found) = find_workspace_name_for_node(child, target_id, workspace_name) {
            return Some(found);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::popup_geometry;
    use crate::config::Position;
    use swayipc::Rect;

    #[test]
    fn center_position_centers_popup_rect() {
        let rect: Rect = serde_json::from_value(serde_json::json!({
            "x": 0,
            "y": 0,
            "width": 2000,
            "height": 1000
        }))
        .unwrap();

        let (width, height, posx, posy) = popup_geometry(rect, &Position::Center, 0.55);

        assert_eq!(width, 880);
        assert_eq!(height, 550);
        assert_eq!(posx, 560);
        assert_eq!(posy, 225);
    }
}
