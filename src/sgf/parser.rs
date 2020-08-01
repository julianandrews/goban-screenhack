use super::errors::SgfParseError;
use super::sgf_node::SgfNode;
use super::props::SgfProp;

pub fn parse(text: &str) -> Result<Vec<SgfNode>, SgfParseError> {
    let mut nodes: Vec<SgfNode> = vec![];
    let mut chars = text.chars();
    loop {
        match chars.next() {
            None => break,
            Some('(') => {
                let (node, text) = parse_game_tree(chars.as_str())?;
                chars = text.chars();
                nodes.push(node);
            },
            Some(c) if c.is_whitespace() => {},
            _ => Err(SgfParseError::InvalidSgf)?,
        }
    }

    Ok(nodes)
}

fn parse_game_tree(text: &str) -> Result<(SgfNode, &str), SgfParseError> {
    if text.chars().next() != Some(';') {
        Err(SgfParseError::InvalidGameTree)?;
    }
    let (mut node, text) = parse_node(&text[1..])?;
    let mut chars = text.chars();
    loop {
        match chars.next() {
            Some(')') => break,
            Some(';') => {
                let (child, text) = parse_node(chars.as_str())?;
                chars = text.chars();
                node.children.push(child);
            },
            Some('(') => {
                let (child, text) = parse_game_tree(chars.as_str())?;
                chars = text.chars();
                node.children.push(child);
            }
            Some(c) if c.is_whitespace() => {},
            _ => Err(SgfParseError::InvalidGameTree)?,
        }
    }

    // TODO: Validate Game Tree level properties
    Ok((node, chars.as_str()))
}

fn parse_node(mut text: &str) -> Result<(SgfNode, &str), SgfParseError> {
    let mut props: Vec<SgfProp> = vec![];
    loop {
        let mut chars = text.chars();
        match chars.next() {
            Some(c) if c.is_ascii_uppercase() => {
                let (prop_ident, new_text) = parse_prop_ident(text)?;
                text = new_text;
                let (prop_values, new_text) = parse_prop_values(text)?;
                text = new_text;
                props.push(SgfProp::new(prop_ident, prop_values)?);
            },
            Some(c) if c.is_whitespace() => text = chars.as_str(),
            _ => break,
        }
    }

    // TODO: Validate Node level properties
    Ok((SgfNode {
        properties: props,
        children: vec![]
    }, text))
}

fn parse_prop_ident(mut text: &str) -> Result<(String, &str), SgfParseError> {
    let mut prop_ident = vec![];
    loop {
        match text.chars().next() {
            Some('[') => break,
            Some(c) if c.is_ascii_uppercase() => {
                prop_ident.push(c);
                text = &text[1..];
            },
            _ => Err(SgfParseError::InvalidProperty)?,
        }
    }

    Ok((prop_ident.iter().collect(), text))
}

fn parse_prop_values(text: &str) -> Result<(Vec<String>, &str), SgfParseError> {
    let mut prop_values = vec![];
    let mut text = text;
    loop {
        let mut chars = text.chars();
        match chars.next() {
            Some('[') => {
                let (value, new_text) = parse_value(chars.as_str())?;
                text = new_text;
                prop_values.push(value);
            },
            Some(c) if c.is_whitespace() => text = chars.as_str(),
            _ => break,
        }
    }

    Ok((prop_values, text))
}

fn parse_value(text: &str) -> Result<(String, &str), SgfParseError> {
    let mut prop_value = vec![];
    let mut chars = text.chars();
    let mut escaped = false;
    loop {
        match chars.next() {
            Some(']') if !escaped => break,
            Some('\\') if !escaped => escaped = true,
            Some(c) => {
                escaped = false;
                prop_value.push(c);
            }
            None => Err(SgfParseError::InvalidProperty)?,
        }
    }

    Ok((prop_value.iter().collect(), chars.as_str()))
}
