use std::error;
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum DomError {
    NoWindow,
    NoDocument,
    UncreatableElement,
    UncreatableNSElement,
    UnappendableElement,

    /// (Attribute, Value)
    UnsetableAttribute(String, String),
    EmptyContainer,

    /// (Id)
    UnfindableId(String),

    /// (Tag Name)
    UnfindableTag(String),

    IdAlreadyExists(String),

    NoParent,

    UnremoveableChild
}

impl error::Error for DomError {
    fn description(&self) -> &str {
        use DomError::*;

        match self {
            NoWindow => "Unable to find window",
            NoDocument => "Unable to find document contained in window",
            UncreatableElement => "Unable to create element in DOM",
            UncreatableNSElement => "Unable to create element with namespace in DOM",
            UnappendableElement => "Unable to append child to container",
            EmptyContainer => "Container which is supposed to contain children does in fact not",
            UnfindableId(_) => "Unable to find id in container",
            UnfindableTag(_) => "Unable to find tag in container",
            UnsetableAttribute(_, _) => "Unable to set attribute",
            IdAlreadyExists(_) => "The given ID already exists within the dom: '{}'",
            NoParent => "Container has no parent element",
            UnremoveableChild => "Unable to remove child"
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for DomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DomError::*;

        match self {
            UnfindableId(arg0) |
            UnfindableTag(arg0) |
            IdAlreadyExists(arg0)
                => write!(f, "Error '{}' with argument '{}'", self.description(), arg0),
            UnsetableAttribute(arg0, arg1)
                => write!(f, "Error '{}' with arguments '{}' and '{}'", self.description(), arg0, arg1),
            _ => write!(f, "Error: {}", self.description())
        }
    }
}

#[derive(Debug, Clone)]
pub enum RendererError {
    UnfindableName(String),
    NameAlreadyExists(String),
    NamedNotContainer(String),
    NamedNotUse(String),
    Dom(DomError)
}

impl error::Error for RendererError {
    fn description(&self) -> &str {
        use RendererError::*;

        match self {
            UnfindableName(_) => "The name is unable to be found",
            NameAlreadyExists(_) => "The name is already being used",
            NamedNotContainer(_) => "The name is not being used for a container",
            NamedNotUse(_) => "The name is not being used for a use element",
            Dom(dom_error) => dom_error.description(),
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RendererError::*;

        match self {
            Dom(dom_error) => write!(f, "{}", dom_error),
            UnfindableName(arg0) |
            NameAlreadyExists(arg0) |
            NamedNotContainer(arg0) |
            NamedNotUse(arg0)
            => write!(f, "Error '{}' with argument '{}'", self.description(), arg0),
        }
    }
}