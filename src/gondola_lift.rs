use crate::ranges_overlap;
use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct EngineSchematic {
    lines: Vec<SchematicLine>,
}

#[derive(Debug, PartialEq)]
struct SchematicLine {
    components: Vec<SchematicComponent>,
}

#[derive(Debug, PartialEq)]
struct SchematicComponent {
    span: Range<usize>,
    component: Component,
}

#[derive(Debug, PartialEq)]
enum Component {
    PartNumber(i32),
    Symbol(char),
    Space,
}

impl FromStr for EngineSchematic {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = Vec::new();
        for line in s.lines() {
            lines.push(line.parse()?)
        }
        Ok(Self { lines })
    }
}

#[derive(Debug, PartialEq)]
pub struct ActivePartNumber {
    part_number: i32,
    line: usize,
    span: Range<usize>,
}

impl ActivePartNumber {
    pub fn part_number(&self) -> i32 {
        self.part_number
    }
}

impl EngineSchematic {
    pub fn get_active_part_numbers(&self) -> Vec<ActivePartNumber> {
        let mut active_part_numbers = Vec::new();
        for (line, schematic_line) in self.lines.iter().enumerate() {
            let prev_line = if line > 0 {
                self.lines.get(line - 1)
            } else {
                None
            };
            let next_line = self.lines.get(line + 1);
            for (i, component) in schematic_line.components.iter().enumerate() {
                let prev_component = if i > 0 {
                    let v = schematic_line.components.get(i - 1).map(|c| &c.component);
                    v
                } else {
                    None
                };
                let next_component = &schematic_line.components.get(i + 1).map(|c| &c.component);
                if let Component::PartNumber(part_number) = component.component {
                    let part_number_span = component.intersect_span();
                    let is_active = if let Some(Component::Symbol(_)) = prev_component {
                        true
                    } else if let Some(Component::Symbol(_)) = next_component {
                        true
                    } else {
                        prev_line.is_some_and(|l| l.has_symbol_overlapping_range(&part_number_span))
                            || next_line
                                .is_some_and(|l| l.has_symbol_overlapping_range(&part_number_span))
                    };
                    if is_active {
                        active_part_numbers.push(ActivePartNumber {
                            part_number,
                            line,
                            span: component.span.clone(),
                        });
                    }
                }
            }
        }

        active_part_numbers
    }
}

impl FromStr for SchematicLine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = Vec::new();
        let mut offset = 0;
        while let Some((component, new_offset)) = SchematicComponent::parse_component(s, offset) {
            components.push(component);
            offset = new_offset;
        }
        Ok(Self { components })
    }
}

impl Display for SchematicLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for component in &self.components {
            write!(f, "{}", component)?;
        }
        Ok(())
    }
}

impl SchematicLine {
    pub fn has_symbol_overlapping_range(&self, range: &Range<usize>) -> bool {
        self.components.iter().any(|c| {
            if let Component::Symbol(_) = c.component {
                ranges_overlap(range, &c.span)
            } else {
                false
            }
        })
    }
}

impl SchematicComponent {
    pub fn len(&self) -> usize {
        self.span.len()
    }

    pub fn intersect_span(&self) -> Range<usize> {
        if self.span.start == 0 {
            self.span.start..self.span.end + 1
        } else {
            self.span.start - 1..self.span.end + 1
        }
    }

    fn parse_component(s: &str, offset: usize) -> Option<(SchematicComponent, usize)> {
        let sub = &s[offset..];
        if sub.is_empty() {
            return None;
        }

        Self::parse_part_number(sub, offset)
            .or_else(|| Self::parse_space(sub, offset))
            .or_else(|| Self::parse_symbol(sub, offset))
    }

    fn parse_part_number(s: &str, offset: usize) -> Option<(SchematicComponent, usize)> {
        let size = s.chars().take_while(|c| c.is_ascii_digit()).count();
        if size > 0 {
            let part_number = s[..size].parse::<i32>().unwrap();
            Some((
                SchematicComponent {
                    span: offset..offset + size,
                    component: Component::PartNumber(part_number),
                },
                offset + size,
            ))
        } else {
            None
        }
    }

    fn parse_symbol(s: &str, offset: usize) -> Option<(SchematicComponent, usize)> {
        let c = s.chars().next()?;
        Some((
            SchematicComponent {
                span: offset..offset + 1,
                component: Component::Symbol(c),
            },
            offset + 1,
        ))
    }

    fn parse_space(s: &str, offset: usize) -> Option<(SchematicComponent, usize)> {
        let size = s.chars().take_while(|c| c == &'.').count();
        if size > 0 {
            Some((
                SchematicComponent {
                    span: offset..offset + size,
                    component: Component::Space,
                },
                offset + size,
            ))
        } else {
            None
        }
    }
}

impl Display for SchematicComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.component {
            Component::PartNumber(part_number) => write!(f, "{}", part_number)?,
            Component::Symbol(symbol) => write!(f, "{}", symbol)?,
            Component::Space => {
                for _ in 0..self.len() {
                    write!(f, ".")?
                }
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_schematic_line() {
        let input = "467..114..";
        let schematic_line = input.parse::<SchematicLine>().unwrap();
        assert_eq!(schematic_line.components.len(), 4);
        assert_eq!(
            schematic_line.components[0].component,
            Component::PartNumber(467)
        );
        assert_eq!(schematic_line.components[1].component, Component::Space);
        assert_eq!(
            schematic_line.components[2].component,
            Component::PartNumber(114)
        );
        assert_eq!(schematic_line.components[3].component, Component::Space);
        assert_eq!(schematic_line.components[0].len(), 3);
        assert_eq!(schematic_line.components[1].len(), 2);
        assert_eq!(schematic_line.components[2].len(), 3);
        assert_eq!(schematic_line.components[3].len(), 2);
        assert_eq!(schematic_line.components[0].span, 0..3);
        assert_eq!(schematic_line.components[1].span, 3..5);
        assert_eq!(schematic_line.components[2].span, 5..8);
        assert_eq!(schematic_line.components[3].span, 8..10);
    }

    #[test]
    fn test_parse_engine_schematic_active_part_numbers() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let schematic = input.parse::<EngineSchematic>().unwrap();
        assert_eq!(schematic.lines.len(), 10);
        assert_eq!(schematic.lines[0].components.len(), 4);
        assert_eq!(schematic.lines[1].components.len(), 3);
        let active_part_numbers = schematic.get_active_part_numbers();
        assert_eq!(
            active_part_numbers.len(),
            8,
            "Number of active part numbers {:?}",
            active_part_numbers
        );
        assert_eq!(active_part_numbers[0].part_number, 467);
        assert_eq!(active_part_numbers[0].line, 0);
        assert_eq!(active_part_numbers[0].span, 0..3);
        let sum_part_numbers = active_part_numbers
            .iter()
            .map(|p| p.part_number)
            .sum::<i32>();
        assert_eq!(sum_part_numbers, 4361);
    }
}
