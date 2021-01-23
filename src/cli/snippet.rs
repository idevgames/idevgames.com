use crate::application_context::ApplicationContext;
use clap::Clap;

/// Grants a permission to a user
#[derive(Debug, Clap)]
pub enum Snippet {
    Delete(Delete),
    SetTaxonomy(SetTaxonomy),
}

impl Snippet {
    pub fn do_the_thing(&self, ctxt: &ApplicationContext) {
        match self {
            Snippet::Delete(d) => d.make_it_go_away(ctxt),
            Snippet::SetTaxonomy(st) => st.do_the_thing(ctxt),
        }
    }
}

#[derive(Debug, Clap)]
pub struct SetTaxonomy {
    /// The id of the snippet.
    id: i32,

    /// The taxonomy to set.
    taxonomy: String,
}

impl SetTaxonomy {
    pub fn do_the_thing(&self, ctxt: &ApplicationContext) {
        let conn = ctxt.db_pool.get().unwrap();
        let mut snippet = crate::models::snippets::Snippet::find_by_id(&conn, self.id).unwrap();
        snippet.taxonomy = self.taxonomy.clone();
        snippet.update(&conn).unwrap();
        println!("Snippet updated!");
    }
}

#[derive(Debug, Clap)]
pub struct Delete {
    id: i32,
}

impl Delete {
    pub fn make_it_go_away(&self, ctxt: &ApplicationContext) {
        let conn = ctxt.db_pool.get().unwrap();
        let snippet = crate::models::snippets::Snippet::find_by_id(&conn, self.id).unwrap();
        snippet.delete(&conn).unwrap();
        println!("Snippet deleted.");
    }
}
