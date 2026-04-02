use std::collections::{BTreeMap, BTreeSet};

use serde_yaml::{Mapping, Value};

#[derive(Debug, Clone)]
pub struct Document {
    pub app: Option<String>,
    pub drill: TreeSection,
    pub inherit: TreeSection,
    pub nav: BTreeMap<String, BTreeSet<String>>,
    pub node: BTreeMap<String, NodeSpec>,
    pub groups: Vec<GroupSpec>,
}

#[derive(Debug, Clone, Default)]
pub struct NodeSpec {
    pub attrs: BTreeMap<String, AttrValue>,
}

#[derive(Debug, Clone)]
pub enum AttrValue {
    Scalar(String),
    Vector(BTreeSet<String>),
}

#[derive(Debug, Clone)]
pub struct GroupSpec {
    pub id: String,
    pub members: BTreeSet<String>,
}

pub type TreeSection = BTreeMap<String, Vec<TreeChild>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeChild {
    Leaf(String),
    Branch(String, Vec<TreeChild>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub message: String,
}

impl ValidationError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub fn parse_document(input: &str) -> Result<Document, ValidationError> {
    let root: Value = serde_yaml::from_str(input)
        .map_err(|err| ValidationError::new(format!("YAML parse error: {err}")))?;
    let root_map = root
        .as_mapping()
        .ok_or_else(|| ValidationError::new("top level must be a mapping"))?;

    let app = optional_string(root_map, "app")?;
    let drill = parse_tree_section(root_map, "drill")?;
    let inherit = parse_tree_section(root_map, "inherit")?;
    let nav = parse_nav_section(root_map, "nav")?;
    let node = parse_node_section(root_map, "node")?;
    let groups = parse_groups(root_map, "groups")?;

    Ok(Document {
        app,
        drill,
        inherit,
        nav,
        node,
        groups,
    })
}

pub fn validate_document(doc: &Document) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    let inherit_leaves = collect_inherit_leaves(&doc.inherit, &mut errors);
    let drill_nodes = collect_all_nodes(&doc.drill, true, &mut errors);
    let pages = inherit_leaves
        .union(&drill_nodes)
        .cloned()
        .collect::<BTreeSet<_>>();

    for node in &drill_nodes {
        if !pages.contains(node) {
            errors.push(ValidationError::new(format!(
                "drill node `{node}` must be page-like"
            )));
        }
    }

    for (nav_id, targets) in &doc.nav {
        if targets.is_empty() {
            errors.push(ValidationError::new(format!(
                "nav `{nav_id}` must not be empty"
            )));
        }
        for target in targets {
            if !pages.contains(target) {
                errors.push(ValidationError::new(format!(
                    "nav `{nav_id}` target `{target}` must be a page"
                )));
            }
        }
    }

    for (node_id, spec) in &doc.node {
        if let Some(AttrValue::Vector(nav_ids)) = spec.attrs.get("nav") {
            for nav_id in nav_ids {
                if !doc.nav.contains_key(nav_id) {
                    errors.push(ValidationError::new(format!(
                        "node `{node_id}` references unknown nav `{nav_id}`"
                    )));
                }
            }
        }
    }

    for group in &doc.groups {
        for member in &group.members {
            if !pages.contains(member) {
                errors.push(ValidationError::new(format!(
                    "group `{}` member `{member}` must be a page",
                    group.id
                )));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn optional_string(root: &Mapping, key: &str) -> Result<Option<String>, ValidationError> {
    match root.get(Value::String(key.to_string())) {
        Some(Value::String(value)) => Ok(Some(value.clone())),
        Some(_) => Err(ValidationError::new(format!("`{key}` must be a string"))),
        None => Ok(None),
    }
}

fn parse_tree_section(root: &Mapping, key: &str) -> Result<TreeSection, ValidationError> {
    let Some(value) = root.get(Value::String(key.to_string())) else {
        return Ok(BTreeMap::new());
    };
    let mapping = value
        .as_mapping()
        .ok_or_else(|| ValidationError::new(format!("`{key}` must be a mapping")))?;
    let mut section = BTreeMap::new();
    for (node_key, children_value) in mapping {
        let node_id = expect_string(node_key, key)?;
        let children = parse_tree_children(children_value, key)?;
        section.insert(node_id, children);
    }
    Ok(section)
}

fn parse_tree_children(value: &Value, section: &str) -> Result<Vec<TreeChild>, ValidationError> {
    let seq = value.as_sequence().ok_or_else(|| {
        ValidationError::new(format!(
            "entries in `{section}` must contain a sequence of children"
        ))
    })?;
    let mut children = Vec::new();
    for item in seq {
        match item {
            Value::String(id) => children.push(TreeChild::Leaf(id.clone())),
            Value::Mapping(map) => {
                if map.len() != 1 {
                    return Err(ValidationError::new(format!(
                        "branch entries in `{section}` must contain exactly one key"
                    )));
                }
                let (branch_key, branch_value) = map.iter().next().expect("single entry");
                let branch_id = expect_string(branch_key, section)?;
                let branch_children = parse_tree_children(branch_value, section)?;
                children.push(TreeChild::Branch(branch_id, branch_children));
            }
            _ => {
                return Err(ValidationError::new(format!(
                    "entries in `{section}` must be strings or single-key mappings"
                )))
            }
        }
    }
    Ok(children)
}

fn parse_nav_section(
    root: &Mapping,
    key: &str,
) -> Result<BTreeMap<String, BTreeSet<String>>, ValidationError> {
    let Some(value) = root.get(Value::String(key.to_string())) else {
        return Ok(BTreeMap::new());
    };
    let mapping = value
        .as_mapping()
        .ok_or_else(|| ValidationError::new(format!("`{key}` must be a mapping")))?;
    let mut navs = BTreeMap::new();
    for (nav_key, nav_value) in mapping {
        let nav_id = expect_string(nav_key, key)?;
        let members = parse_string_set(nav_value, &format!("nav `{nav_id}`"))?;
        navs.insert(nav_id, members);
    }
    Ok(navs)
}

fn parse_node_section(
    root: &Mapping,
    key: &str,
) -> Result<BTreeMap<String, NodeSpec>, ValidationError> {
    let Some(value) = root.get(Value::String(key.to_string())) else {
        return Ok(BTreeMap::new());
    };
    let mapping = value
        .as_mapping()
        .ok_or_else(|| ValidationError::new(format!("`{key}` must be a mapping")))?;
    let mut nodes = BTreeMap::new();
    for (node_key, node_value) in mapping {
        let node_id = expect_string(node_key, key)?;
        let attrs_map = node_value.as_mapping().ok_or_else(|| {
            ValidationError::new(format!("node `{node_id}` must be a mapping of attributes"))
        })?;
        let mut attrs = BTreeMap::new();
        for (attr_key, attr_value) in attrs_map {
            let attr_name = expect_string(attr_key, &format!("node `{node_id}`"))?;
            let parsed = match attr_value {
                Value::Sequence(_) => AttrValue::Vector(parse_string_set(
                    attr_value,
                    &format!("node `{node_id}` attribute `{attr_name}`"),
                )?),
                Value::String(value) => AttrValue::Scalar(value.clone()),
                Value::Number(value) => AttrValue::Scalar(value.to_string()),
                Value::Bool(value) => AttrValue::Scalar(value.to_string()),
                Value::Null => AttrValue::Scalar("null".to_string()),
                Value::Mapping(_) => {
                    return Err(ValidationError::new(format!(
                        "node `{node_id}` attribute `{attr_name}` must be scalar or vector"
                    )))
                }
                Value::Tagged(_) => {
                    return Err(ValidationError::new(format!(
                        "node `{node_id}` attribute `{attr_name}` must not use tags"
                    )))
                }
            };
            attrs.insert(attr_name, parsed);
        }
        nodes.insert(node_id, NodeSpec { attrs });
    }
    Ok(nodes)
}

fn parse_groups(root: &Mapping, key: &str) -> Result<Vec<GroupSpec>, ValidationError> {
    let Some(value) = root.get(Value::String(key.to_string())) else {
        return Ok(Vec::new());
    };
    let seq = value
        .as_sequence()
        .ok_or_else(|| ValidationError::new(format!("`{key}` must be a sequence")))?;
    let mut groups = Vec::new();
    for item in seq {
        let map = item
            .as_mapping()
            .ok_or_else(|| ValidationError::new("group entries must be mappings"))?;
        let id = required_string(map, "id", "group")?;
        let members_value = map
            .get(Value::String("members".to_string()))
            .ok_or_else(|| ValidationError::new(format!("group `{id}` must define `members`")))?;
        let members = parse_string_set(members_value, &format!("group `{id}` members"))?;
        groups.push(GroupSpec { id, members });
    }
    Ok(groups)
}

fn parse_string_set(value: &Value, context: &str) -> Result<BTreeSet<String>, ValidationError> {
    let seq = value
        .as_sequence()
        .ok_or_else(|| ValidationError::new(format!("{context} must be a sequence")))?;
    let mut out = BTreeSet::new();
    for item in seq {
        let id = item
            .as_str()
            .ok_or_else(|| ValidationError::new(format!("{context} must contain only strings")))?;
        out.insert(id.to_string());
    }
    Ok(out)
}

fn expect_string(value: &Value, context: &str) -> Result<String, ValidationError> {
    value
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| ValidationError::new(format!("keys in `{context}` must be strings")))
}

fn required_string(map: &Mapping, key: &str, context: &str) -> Result<String, ValidationError> {
    map.get(Value::String(key.to_string()))
        .ok_or_else(|| ValidationError::new(format!("{context} must define `{key}`")))?
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| ValidationError::new(format!("{context} field `{key}` must be a string")))
}

fn collect_inherit_leaves(
    section: &TreeSection,
    errors: &mut Vec<ValidationError>,
) -> BTreeSet<String> {
    let mut leaves = BTreeSet::new();
    for (root, children) in section {
        collect_inherit_leaves_children(root, children, &mut leaves, errors, true);
    }
    leaves
}

fn collect_inherit_leaves_children(
    current: &str,
    children: &[TreeChild],
    leaves: &mut BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
    current_is_non_leaf: bool,
) {
    if children.is_empty() && !current_is_non_leaf && !leaves.insert(current.to_string()) {
        errors.push(ValidationError::new(format!(
            "inherit leaf `{current}` appears more than once"
        )));
    }
    for child in children {
        match child {
            TreeChild::Leaf(id) => {
                if !leaves.insert(id.clone()) {
                    errors.push(ValidationError::new(format!(
                        "inherit leaf `{id}` appears more than once"
                    )));
                }
            }
            TreeChild::Branch(id, grand_children) => {
                collect_inherit_leaves_children(id, grand_children, leaves, errors, false);
            }
        }
    }
}

fn collect_all_nodes(
    section: &TreeSection,
    include_roots: bool,
    errors: &mut Vec<ValidationError>,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for (root, children) in section {
        if include_roots && !out.insert(root.clone()) {
            errors.push(ValidationError::new(format!(
                "drill node `{root}` appears more than once"
            )));
        }
        collect_nodes_children(children, &mut out, errors, root.as_str(), include_roots);
    }
    out
}

fn collect_nodes_children(
    children: &[TreeChild],
    out: &mut BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
    _parent: &str,
    include_branches: bool,
) {
    for child in children {
        match child {
            TreeChild::Leaf(id) => {
                if !out.insert(id.clone()) {
                    errors.push(ValidationError::new(format!(
                        "drill node `{id}` appears more than once"
                    )));
                }
            }
            TreeChild::Branch(id, nested) => {
                if include_branches && !out.insert(id.clone()) {
                    errors.push(ValidationError::new(format!(
                        "drill node `{id}` appears more than once"
                    )));
                } else if !include_branches && !out.insert(id.clone()) {
                    errors.push(ValidationError::new(format!(
                        "drill node `{id}` appears more than once"
                    )));
                }
                collect_nodes_children(nested, out, errors, id, include_branches);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_document, validate_document};

    const DEMO: &str = include_str!("../examples/demo.gui");

    #[test]
    fn parses_and_validates_demo() {
        let doc = parse_document(DEMO).expect("parse demo");
        validate_document(&doc).expect("validate demo");
    }

    #[test]
    fn rejects_drill_node_missing_from_inherit_leaves() {
        let src = r#"
app: Bad
drill:
  Home:
    - Missing
inherit:
  RootLayout:
    - Home
nav:
  GlobalNav: [Home, Ghost]
node:
  Home:
    path: /
"#;
        let doc = parse_document(src).expect("parse");
        let errors = validate_document(&doc).expect_err("should fail");
        assert!(errors.iter().any(|err| err.message.contains("Ghost")));
    }
}
