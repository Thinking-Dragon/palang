use ptree::{write_tree, TreeBuilder};

use super::{assembly::Assembly, function::Function, model::Model, prompt::Prompt};

#[derive(Debug)]
pub enum AssemblyDependencyNode {
    Root {
        children: Vec<Box<Self>>
    },
    Package {
        name: String,
        children: Vec<Box<Self>>,
    },
    Model(Model),
    Prompt(Prompt),
    Function(Function),
}

impl AssemblyDependencyNode {
    pub fn from_assemblies(assemblies: &Vec<Assembly>) -> Self {
        let mut root = AssemblyDependencyNode::Root {
            children: Vec::new(),
        };

        for assembly in assemblies {
            insert_assembly(&mut root, assembly);
        }

        root
    }

    pub fn from_assembly(assembly: &Assembly) -> Self {
        let mut root = AssemblyDependencyNode::Root {
            children: Vec::new(),
        };

        insert_assembly(&mut root, assembly);

        root
    }

    pub fn to_string(&self) -> Result<String, String> {
        if let AssemblyDependencyNode::Root { .. } = self {
            let mut buffer: Vec<u8> = Vec::new();
            let mut tree: TreeBuilder = TreeBuilder::new("Assemblies:".to_string());
            self.build_tree(&mut tree);

            match write_tree(&tree.build(), &mut buffer) {
                Ok(_) => {
                    match String::from_utf8(buffer) {
                        Ok(s) => Ok(s),
                        Err(e) => Err(format!("Failed to convert buffer to string: {}", e))
                    }
                },
                Err(e) => Err(format!("Failed to write tree: {}", e))
            }
        } else {
            Err("Dependency tree must begin with a Root".to_string())
        }
    }

    fn build_tree(&self, tree: &mut TreeBuilder) {
        match self {
            AssemblyDependencyNode::Root { children, .. } => {
                for child in children {
                    child.build_tree(tree);
                }
            }
            AssemblyDependencyNode::Package { name, children } => {
                tree.begin_child(name.clone());
                for child in children {
                    child.build_tree(tree);
                }
                tree.end_child();
            }
            AssemblyDependencyNode::Model(model) => {
                match model {
                    Model::Litteral(model) => {
                        tree.add_empty_child(format!("{} (model)", get_name_from_path(&model.name)));
                    },
                    Model::Enumerable(model) => {
                        let name: String = get_name_from_path(&model.name);
                        let possible_models: String = model.possible_models.iter()
                            .map(|model| get_name_from_path(model))
                            .collect::<Vec<String>>()
                            .join(", ");

                        tree.add_empty_child(format!("{} [{}] (model)",  name, possible_models));
                    },
                }
            }
            AssemblyDependencyNode::Prompt(prompt) => {
                tree.add_empty_child(format!("{} (prompt)", get_name_from_path(&prompt.name)));
            }
            AssemblyDependencyNode::Function(function) => {
                tree.add_empty_child(format!("{} (function)", get_name_from_path(&function.name)));
            }
        }
    }
}

impl Assembly {
    pub fn get_dependency_tree(&self) -> AssemblyDependencyNode {
        AssemblyDependencyNode::from_assembly(self)
    }
}

fn insert_assembly(root: &mut AssemblyDependencyNode, assembly: &Assembly) {
    for (path, model) in &assembly.models {
        let path_parts: Vec<&str> = path.split('/').collect();
        insert_node(root, path_parts[..path_parts.len() - 1].to_vec(), AssemblyDependencyNode::Model(model.clone()));
    }

    for (path, prompt) in &assembly.prompts {
        let path_parts: Vec<&str> = path.split('/').collect();
        insert_node(root, path_parts[..path_parts.len() - 1].to_vec(), AssemblyDependencyNode::Prompt(prompt.clone()));
    }

    for (path, function) in &assembly.functions {
        let path_parts: Vec<&str> = path.split('/').collect();
        insert_node(root, path_parts[..path_parts.len() - 1].to_vec(), AssemblyDependencyNode::Function(function.clone()));
    }
}

fn insert_node(current: &mut AssemblyDependencyNode, path: Vec<&str>, node: AssemblyDependencyNode) {
    match current {
        AssemblyDependencyNode::Root { children, .. } |
        AssemblyDependencyNode::Package { children, .. } => {
            if path.is_empty() {
                children.push(Box::new(node));
            } else {
                let package_name = path[0];
                let child = children.iter_mut()
                    .find(|child| matches!(
                        child.as_ref(),
                        AssemblyDependencyNode::Package { name, .. }
                        if name == package_name
                    )
                );

                if let Some(child) = child {
                    insert_node(child, path[1..].to_vec(), node);
                } else {
                    let mut new_package = AssemblyDependencyNode::Package {
                        name: package_name.to_string(),
                        children: Vec::new(),
                    };
                    insert_node(&mut new_package, path[1..].to_vec(), node);
                    children.push(Box::new(new_package));
                }
            }
        }
        _ => panic!("Unexpected node type"),
    }
}

fn get_name_from_path(path: &String) -> String {
    match path.rfind('/') {
        Some(last_slash) => path[last_slash + 1..].to_string(),
        None => path.clone(),
    }
}
