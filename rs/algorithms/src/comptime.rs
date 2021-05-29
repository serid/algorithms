#![allow(dead_code)]

mod base {
    use std::marker::PhantomData;

    pub trait Bool {
        fn reify() -> bool;
    }

    pub struct False;

    pub struct True;

    impl Bool for True {
        fn reify() -> bool {
            true
        }
    }

    impl Bool for False {
        fn reify() -> bool {
            false
        }
    }


    pub trait OptionT<V> {}

    pub struct OptionTNone {}

    pub struct OptionTSome<V>(PhantomData<V>);

    impl<V> OptionT<V> for OptionTNone {}

    impl<V> OptionT<V> for OptionTSome<V> {}

    pub trait OptionTUnwrap<V>: OptionT<V> {
        type Out;
    }

    impl<V> OptionTUnwrap<V> for OptionTSome<V> {
        type Out = V;
    }
}

/*
/// Type comparison trait
pub trait TEq {
    type Other: ?Sized;
}

impl<T: ?Sized> TEq for T {
    type Other = Self;
}

fn test_equality<T, U: TEq<Other=T>>() {}

*/

// pub trait TEqCoq<X, Y> {}
//
// pub struct TEqCoqS<X>(PhantomData<X>);
//
// impl<X> TEqCoq<X, X> for TEqCoqS<X> {}

/// Type-level Heterogeneous List
#[macro_use]
mod hlist {
    use std::marker::PhantomData;

    use super::base::*;
    use super::peano::*;

    pub struct HNil;

    pub struct HCons<T, R: HList>(T, R);

    pub trait HList {}

    impl HList for HNil {}

    impl<T, R: HList> HList for HCons<T, R> {}

    macro_rules! hlister {
        ( $x:ty, $( $y:ty ),* $(,)* ) => {
            HCons<$x, hlister![$($y, )*]>
        };

        () => {
            HNil
        }
    }

    mod some_experiments {
        /*
        trait True {}

        struct I;

        impl True for I {}

        trait Or<T, U> {}

        struct Left<T>(PhantomData<T>);

        struct Right<T>(PhantomData<T>);

        impl<T, U> Or<T, U> for Left<T> {}

        impl<T, U> Or<T, U> for Right<U> {}


        trait And<T, U> {}

        struct Conj<T, U>(PhantomData<(T, U)>);

        impl<T, U> And<T, U> for Conj<T, U> {}


        trait IsZero<N: Peano> {}

        struct IsZeroS<N: Peano, E: Teq3<N, PO>>(PhantomData<(N, E)>);
        */

        /* // Type Equality
        pub trait TT<T, U, B: Bool> {}

        impl<T> TT<T, T, True> for () {}

        impl<T, U> TT<T, U, False> for () {}

        fn a<T: TT<i32, i32, False>>() {
            a::<()>();
        }
        */
    }

    /// Get a type-value from HList by index
    pub trait Get<I: Peano, List: HList, R> {}

    impl<I: Peano> Get<I, HNil, OptionTNone> for () {}

    impl<V, Tail: HList> Get<PO, HCons<V, Tail>, OptionTSome<V>> for () {}

    impl<Is: Peano, V, Tail: HList, R> Get<PS<Is>, HCons<V, Tail>, R> for () where
        (): Get<Is, Tail, R> {}

    /// Get type-value and put it in ::Out
    pub trait GetRes<I: Peano, List: HList, V, R: OptionT<V>> {
        type Out;
    }

    impl<I: Peano, List: HList, V, R: OptionT<V>> GetRes<I, List, V, R> for () where
        (): Get<I, List, R> {
        type Out = R;
    }

    /// GetRes with unwrap
    pub trait GetResUnwrap<I: Peano, List: HList, V, R: OptionT<V>> {
        type Out;
    }

    impl<I: Peano, List: HList, V, R: OptionT<V> + OptionTUnwrap<V>> GetResUnwrap<I, List, V, R> for () where
        (): Get<I, List, R> {
        type Out = <R as OptionTUnwrap<V>>::Out;
    }

    pub(crate) type GGet<I, List, V, R> = <() as GetResUnwrap<I, List, V, R>>::Out;


    /// Determines whether N is a member of HList
    pub trait Member<N: Peano, List: HList, R: Or> {}

    impl<N: Peano> Member<N, HNil, Just<False>> for () {}

    impl<N: Peano, M: Peano, Tail: HList, R1: Bool, R2: Or> Member<N, HCons<M, Tail>, Disj<Just<R1>, R2>> for () where
        (): PeanoEq<N, M, R1>, (): Member<N, Tail, R2>, Disj<Just<R1>, R2>: Or {}

    pub fn test_member<N: Peano, L: HList, Ok: Or + RunOr>() where
        (): Member<N, L, Ok> {
        println!("{}", <Ok as RunOr>::Out::reify());
    }

    /// Tree of disjunctions and atomic booleans
    pub trait Or {}

    pub struct Disj<A: Or, B: Or>(PhantomData<(A, B)>);

    pub struct Just<B: Bool>(PhantomData<B>);

    impl<A: Or, B: Or> Or for Disj<A, B> {}

    impl<B: Bool> Or for Just<B> {}


    /// Computes the Bool value of an Or tree
    pub trait RunOr: Or {
        type Out: Bool;
    }

    impl RunOr for Just<False> {
        type Out = False;
    }

    impl RunOr for Just<True> {
        type Out = True;
    }

    impl<A: RunOr, B: RunOr> RunOr for Disj<A, B> where
        (): Or2<A::Out, B::Out> {
        type Out = <() as Or2<A::Out, B::Out>>::Out;
    }

    /// Or operation on booleans
    pub trait Or2<A: Bool, B: Bool> {
        type Out: Bool;
    }

    impl Or2<False, False> for () {
        type Out = False;
    }

    impl Or2<False, True> for () {
        type Out = True;
    }

    impl Or2<True, False> for () {
        type Out = True;
    }

    impl Or2<True, True> for () {
        type Out = True;
    }
}

mod peano {
    use std::marker::PhantomData;

    use super::base::*;

    pub struct PO;

    pub struct PS<N: Peano>(PhantomData<N>);

    pub trait Peano {
        fn reify() -> i64;
    }

    impl Peano for PO {
        fn reify() -> i64 {
            0
        }
    }

    impl<P: Peano> Peano for PS<P> {
        fn reify() -> i64 {
            1 + P::reify()
        }
    }

    pub type Zero = PO;
    pub type One = PS<PO>;
    pub type Two = PS<PS<PO>>;
    pub type Three = PS<PS<PS<PO>>>;
    pub type Four = PS<Three>;
    pub type Five = PS<Four>;
    pub type Six = PS<Five>;
    pub type Seven = PS<Six>;
    pub type Eight = PS<Seven>;
    pub type Nine = PS<Eight>;
    pub type Ten = PS<Nine>;

    pub type Hundred = MulT<Ten, Ten>;
    pub type Thousand = MulT<Ten, Hundred>;
    pub type Million = MulT<Thousand, Thousand>;


    /// Equality relation for peano numbers
    pub trait PeanoEq<N: Peano, M: Peano, R: Bool> {}

    impl<> PeanoEq<PO, PO, True> for () {}

    impl<N: Peano> PeanoEq<PS<N>, PO, False> for () {}

    impl<N: Peano> PeanoEq<PO, PS<N>, False> for () {}

    impl<N: Peano, M: Peano, R: Bool> PeanoEq<PS<N>, PS<M>, R> for ()
        where (): PeanoEq<N, M, R> {}


    /// Three implementations of natural summation

    /// Logical predicate of summation (Good)
    pub trait Plus<N: Peano, M: Peano, Sum: Peano> {}

    impl<N: Peano> Plus<PO, N, N> for () {}

    impl<N: Peano, M: Peano, Sum: Peano> Plus<PS<N>, M, PS<Sum>> for () where (): Plus<N, M, Sum> {}

    /// Ugly summation function where first argument is Self (: Peano) (The worst)
    pub trait Plus2<M: Peano>: Peano {
        type Out: Peano;
    }

    impl<M: Peano> Plus2<M> for PO {
        type Out = M;
    }

    impl<N: Peano, M: Peano> Plus2<M> for PS<N> where
        N: Plus2<M>,
    {
        type Out = PS<<N as Plus2<M>>::Out>;
    }

    /// Summation function 2 (The best)
    pub trait Plus3<N: Peano, M: Peano> {
        type Out: Peano;
    }

    impl<M: Peano> Plus3<PO, M> for () {
        type Out = M;
    }

    impl<N: Peano, M: Peano> Plus3<PS<N>, M> for () where
        (): Plus3<N, M>,
    {
        type Out = PS<<() as Plus3<N, M>>::Out>;
    }

    /// Logical predicate usage
    pub fn sum<N: Peano, M: Peano, Sum: Peano>() -> i64 where
        (): Plus<N, M, Sum>,
    {
        Sum::reify()
    }

    /// Summation function usage
    pub fn sum2<N: Plus2<M>, M: Peano>() -> i64 {
        N::Out::reify()
    }

    /// Summation function 2 usage
    pub fn sum3<N: Peano, M: Peano>() -> i64 where
        (): Plus3<N, M>,
    {
        <() as Plus3<N, M>>::Out::reify()
    }

    /// Summation function 2 usage 2. Allows running inverse function
    pub fn sum4<N: Peano, M: Peano, Sum: Peano>() -> i64 where
        (): Plus3<N, M, Out=Sum>,
    {
        M::reify()
    }

    /// Multiplication function
    pub trait Mul<N: Peano, M: Peano> {
        type Out: Peano;
    }

    impl<M: Peano> Mul<PO, M> for () {
        type Out = PO;
    }

    impl<N: Peano, M: Peano> Mul<PS<N>, M> for () where
        (): Mul<N, M>,
        (): Plus3<<() as Mul<N, M>>::Out, M>,
    {
        type Out = <() as Plus3<<() as Mul<N, M>>::Out, M>>::Out;
    }

    /// Mul function usage. Allows running inverse function
    pub fn mul<N: Peano, M: Peano, Prod: Peano>() -> i64 where
        (): Mul<N, M, Out=Prod>,
    {
        Prod::reify()
    }

    pub type MulT<N, M> = <() as Mul<N, M>>::Out;
}

fn main() {
    use base::*;
    use peano::*;

    // println!("Number: {}", sum::<Two, Three, _>());
    // println!("Number: {}", sum2::<Six, Three>());
    // println!("Number: {}", sum3::<Four, Six>());
    // println!("Number: {}", sum4::<Four, _, Six>());
    // println!("Number: {}", mul::<Three, Five, _>());
    // println!("Number: {}", mul::<Three, Five, _>());
    // println!("Number: {}", Hundred::reify());

    use hlist::*;

    let _a: GGet<Zero, hlister![i32, char, String], _, _> = 100;

    let _a: <<() as GetRes<PO, HCons<i32, HNil>, _, _>>::Out as OptionTUnwrap<_>>::Out = 100;



    test_member::<Five, hlister![Four, Five, Two], _>();
    test_member::<Two, hlister![Ten, One, Six], _>();

    // println!("Hello, world!");
}
