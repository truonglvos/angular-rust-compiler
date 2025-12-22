// Decorator Extractor
//
// Extracts Angular decorator documentation.

use super::entities::*;

/// Decorator types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecoratorType {
    Component,
    Directive,
    Pipe,
    NgModule,
    Injectable,
    Input,
    Output,
    HostBinding,
    HostListener,
    ViewChild,
    ViewChildren,
    ContentChild,
    ContentChildren,
}

/// Extracts Angular decorator documentation.
pub struct DecoratorExtractor;

impl DecoratorExtractor {
    /// Check if a decorator name is an Angular decorator.
    pub fn is_angular_decorator(name: &str) -> bool {
        matches!(name, 
            "Component" | "Directive" | "Pipe" | "NgModule" | 
            "Injectable" | "Input" | "Output" | "HostBinding" |
            "HostListener" | "ViewChild" | "ViewChildren" |
            "ContentChild" | "ContentChildren"
        )
    }
    
    /// Get decorator type from name.
    pub fn get_decorator_type(name: &str) -> Option<DecoratorType> {
        match name {
            "Component" => Some(DecoratorType::Component),
            "Directive" => Some(DecoratorType::Directive),
            "Pipe" => Some(DecoratorType::Pipe),
            "NgModule" => Some(DecoratorType::NgModule),
            "Injectable" => Some(DecoratorType::Injectable),
            "Input" => Some(DecoratorType::Input),
            "Output" => Some(DecoratorType::Output),
            "HostBinding" => Some(DecoratorType::HostBinding),
            "HostListener" => Some(DecoratorType::HostListener),
            "ViewChild" => Some(DecoratorType::ViewChild),
            "ViewChildren" => Some(DecoratorType::ViewChildren),
            "ContentChild" => Some(DecoratorType::ContentChild),
            "ContentChildren" => Some(DecoratorType::ContentChildren),
            _ => None,
        }
    }
    
    /// Extract decorator entry.
    pub fn extract(name: &str, decorator_type: DecoratorType) -> DocEntry {
        let entry_type = match decorator_type {
            DecoratorType::Component => EntryType::Component,
            DecoratorType::Directive => EntryType::Directive,
            DecoratorType::Pipe => EntryType::Pipe,
            DecoratorType::NgModule => EntryType::NgModule,
            DecoratorType::Injectable => EntryType::Injectable,
            _ => EntryType::Decorator,
        };
        
        DocEntry::new(name, entry_type)
    }
}
