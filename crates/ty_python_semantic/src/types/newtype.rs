use std::borrow::Cow;

use ruff_python_ast::name::Name;

use crate::{
    Db,
    semantic_index::definition::Definition,
    types::{ClassType, Type, constraints::Constraints, tuple::TupleSpec},
};

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, get_size2::GetSize, salsa::Update, PartialOrd, Ord,
)]
pub struct NewTypePseudoClass<'db>(NewType<'db>);

impl<'db> NewTypePseudoClass<'db> {
    pub(crate) fn from_class(
        db: &'db dyn Db,
        name: &str,
        supertype: ClassType<'db>,
        definition: Definition<'db>,
    ) -> Self {
        Self(NewType::new(
            db,
            Name::from(name),
            NewTypeBase::Class(supertype),
            definition,
        ))
    }

    pub(crate) fn from_new_type(
        db: &'db dyn Db,
        name: &str,
        supertype: NewTypePseudoClass<'db>,
        definition: Definition<'db>,
    ) -> Self {
        Self(NewType::new(
            db,
            Name::from(name),
            NewTypeBase::NewType(supertype.0),
            definition,
        ))
    }

    pub(crate) fn to_instance(&self) -> NewTypeInstance<'db> {
        NewTypeInstance(self.0)
    }

    pub(crate) fn name(&self, db: &'db dyn Db) -> &'db str {
        self.0.name(db)
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, get_size2::GetSize, salsa::Update, PartialOrd, Ord,
)]
pub struct NewTypeInstance<'db>(NewType<'db>);

impl<'db> NewTypeInstance<'db> {
    pub(crate) fn tuple_spec(self, db: &'db dyn Db) -> Option<Cow<TupleSpec<'db>>> {
        match self.0.supertype(db) {
            NewTypeBase::Class(class) => Type::instance(db, class).tuple_instance_spec(db),
            NewTypeBase::NewType(newtype) => NewTypeInstance(newtype).tuple_spec(db),
        }
    }

    pub(crate) fn supertype(self, db: &'db dyn Db) -> Type<'db> {
        match self.0.supertype(db) {
            NewTypeBase::Class(class) => Type::instance(db, class),
            NewTypeBase::NewType(newtype) => Type::NewTypeInstance(NewTypeInstance(newtype)),
        }
    }

    pub(crate) fn has_relation_to<C: Constraints<'db>>(
        self,
        db: &'db dyn Db,
        other: NewTypeInstance<'db>,
    ) -> C {
        if self == other {
            return C::from_bool(db, true);
        }
        match self.0.supertype(db) {
            NewTypeBase::Class(_) => C::from_bool(db, false),
            NewTypeBase::NewType(newtype) => NewTypeInstance(newtype).has_relation_to(db, other),
        }
    }

    pub(crate) fn is_disjoint_from<C: Constraints<'db>>(
        self,
        db: &'db dyn Db,
        other: NewTypeInstance<'db>,
    ) -> C {
        C::from_bool(
            db,
            !(self.has_relation_to(db, other) || other.has_relation_to(db, self)),
        )
    }

    pub(crate) fn name(&self, db: &'db dyn Db) -> &'db str {
        self.0.name(db)
    }
}

#[salsa::interned(debug, heap_size=ruff_memory_usage::heap_size)]
#[derive(PartialOrd, Ord)]
struct NewType<'db> {
    #[returns(ref)]
    name: Name,
    supertype: NewTypeBase<'db>,
    definition: Definition<'db>,
}

// The Salsa heap is tracked separately.
impl get_size2::GetSize for NewType<'_> {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, get_size2::GetSize)]
enum NewTypeBase<'db> {
    Class(ClassType<'db>),
    NewType(NewType<'db>),
}
