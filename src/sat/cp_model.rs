// IMPORT CXX LIBRARY
cpp! {{
    #include "ortools/sat/cp_model.h"
}}

pub(crate) mod wrap {
    #[doc(hidden)]
    pub trait Expr {
        fn into_ptr(self) -> *mut ()
        where
            Self: Sized;
    }

    impl<T> Expr for Box<T>
    where
        T: Expr,
    {
        fn into_ptr(self) -> *mut () {
            Box::leak(self) as *mut T as *mut ()
        }
    }
}

cpp_class!(
    /// An integer variable.
    ///
    /// This class wraps an IntegerVariableProto.
    /// This can only be constructed via \c CpModelBuilder.NewIntVar().
    ///
    pub unsafe struct IntVar as "operations_research::sat::IntVar"
);

impl self::wrap::Expr for IntVar {
    fn into_ptr(mut self) -> *mut () {
        &mut self as *mut IntVar as *mut ()
    }
}

cpp_class!(
    /// A constraint.
    ///
    /// This class enables you to modify the constraint that was previously added to
    /// the model.
    ///
    /// The constraint must be built using the different `CpModelBuilder::AddXXX`
    /// methods.
    ///
    pub unsafe struct Constraint as "operations_research::sat::Constraint"
);
