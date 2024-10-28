mod private
{
  use crate::*;
  use std::panic::UnwindSafe;

  /// Ordering of scalars in buffer of matrix, either row-major or column-major.
  pub trait Order
  {
    /// True for row-major
    const IS_ROW_MAJOR: bool;
    /// Name of order
    fn order_str() -> &'static str;
  }
  /// Row-major ordering of scalars in buffer of matrix.
  pub trait OrderRowMajor : Order
  {
    /// True for row-major
    const IS_ROW_MAJOR: bool = true;
    /// Name of order
    #[ inline( always ) ]
    fn order_str() -> &'static str
    {
      "row-major"
    }
  }
  /// Column-major ordering of scalars in buffer of matrix.
  pub trait OrderColumnMajor : Order
  {
    /// True for row-major
    const IS_ROW_MAJOR: bool = false;
    /// Name of order
    #[ inline( always ) ]
    fn order_str() -> &'static str
    {
      "column-major"
    }
  }

  /// Descriptor of a matrix, which describe coordinate system used, its ordering ( row-major/column-major ) and other attributes.
  pub trait Descriptor : Order + UnwindSafe
  {
    /// True for row-major ordering
    const IS_ROW_MAJOR: bool;
    /// True for ordinary coordinates
    const IS_ORDINARY: bool;

    /// Coordinate type of the matrix( homogenous or not)
    #[ inline( always ) ]
    fn coords_str() -> &'static str
    {
      if Self::IS_ORDINARY
      {
        "ordinary"
      }
      else 
      {
        "homogenous"
      }
    }
  }

  // =

  /// Ordinary coordinates with row-major ordering.
  #[ derive( Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug, Display ) ]
  #[ display( "DescriptorOrderRowMajor" ) ]
  pub struct DescriptorOrderRowMajor;
  impl Descriptor for DescriptorOrderRowMajor {
    const IS_ROW_MAJOR: bool = true;
    const IS_ORDINARY: bool = true;
  }
  impl Order for DescriptorOrderRowMajor
  {
    const IS_ROW_MAJOR: bool = true;
    #[ inline( always ) ]
    fn order_str() -> &'static str
    {
      "row-major"
    }
  }
  impl OrderRowMajor for DescriptorOrderRowMajor {}

  /// Default matrix descriptor.
  /// It's ordinary coordinates with row-major ordering.
  pub type DescriptorDefault = DescriptorOrderRowMajor;

  // /// Homogenous coordinates with row-major ordering.
  // #[ derive( Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug, Display ) ]
  // #[ display( "DescriptorOrderRowMajor" ) ]
  // pub struct DescriptorOrderRowMajor;
  // // impl Descriptor for DescriptorOrderRowMajor {}

  // = DescriptorOrderColumnMajor

  /// Ordinary coordinates with column-major ordering.
  #[ derive( Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug, Display ) ]
  #[ display( "DescriptorOrderColumnMajor" ) ]
  pub struct DescriptorOrderColumnMajor;
  impl Descriptor for DescriptorOrderColumnMajor {
    const IS_ROW_MAJOR: bool = false;
    const IS_ORDINARY: bool = true;
  }
  impl Order for DescriptorOrderColumnMajor
  {
    const IS_ROW_MAJOR: bool = false;
    #[ inline( always ) ]
    fn order_str() -> &'static str
    {
      "column-major"
    }
  }
  impl OrderColumnMajor for DescriptorOrderColumnMajor {}

  // =

  pub trait MatEl
  where
    Self : Copy + Default,
  {
  }

  impl< T > MatEl for T
  where
    Self : Copy + Default,
  {
  }

  // =

  /// A matrix structure.
  #[ derive( From, Clone, Copy, PartialEq, PartialOrd, Hash ) ]
  // pub struct Mat< const ROWS : usize, const COLS : usize, E = f32, Descriptor : mat::Descriptor = DescriptorDefault >
  pub struct Mat< const ROWS : usize, const COLS : usize, E, Descriptor >
  (
    [ [ E ; COLS ] ; ROWS ],
    core::marker::PhantomData< Descriptor >,
  )
  where
    E : MatEl,
  ;

  impl< E, const ROWS : usize, const COLS : usize, Descriptor : mat::Descriptor > Mat< ROWS, COLS, E, Descriptor >
  where
    E : MatEl,
  {

    #[ inline( always ) ]
    fn _new( raw_slice : [ [ E ; COLS ] ; ROWS ] ) -> Self
    {
      Mat( raw_slice, Default::default() )
    }

    /// Fill matrix with a scalar value.
    #[ inline( always ) ]
    pub fn _fill( scalar : E ) -> Self
    where
      E : MatEl,
    {
      Self::_new( [ [ scalar ; COLS ] ; ROWS ] )
    }

    /// Returns a raw pointer to the matrix buffer.
    #[ inline( always ) ]
    pub const fn as_ptr( &self ) -> *const E
    {
      self.0.as_ptr() as *const E
    }

    /// Returns an unsafe mutable pointer to the slice's buffer.
    #[ inline( always ) ]
    pub fn as_mut_ptr( &mut self ) -> *mut E
    {
      self.0.as_mut_ptr() as *mut E
    }

    /// Returns the number of rows in the matrix.
    #[ inline( always ) ]
    pub const fn rows( &self ) -> usize
    {
      ROWS
    }

    /// Returns the number of columns in the matrix.
    #[ inline( always ) ]
    pub const fn cols( &self ) -> usize
    {
      COLS
    }

  }

  impl< E, const ROWS : usize, const COLS : usize, Descriptor : mat::Descriptor > Default for Mat< ROWS, COLS, E, Descriptor >
  where
    E : MatEl,
  {
    #[ inline( always ) ]
    fn default() -> Self
    {
      Mat( [ [ E::default() ; COLS ]; ROWS ], Default::default() )
    }
  }

  impl< E, const ROWS : usize, const COLS : usize, Descriptor : mat::Descriptor > Collection
  for Mat< ROWS, COLS, E, Descriptor >
  where
    E : MatEl,
  {
    type Scalar = E;
  }

  impl< E, const ROWS : usize, const COLS : usize, Descriptor : mat::Descriptor > MatWithShape< ROWS, COLS >
  for Mat< ROWS, COLS, E, Descriptor >
  where
    E : MatEl,
  {
  }

  impl< E, const ROWS : usize, const COLS : usize, Descriptor : mat::Descriptor > MatWithShapeMut< ROWS, COLS >
  for Mat< ROWS, COLS, E, Descriptor >
  where
    E : MatEl,
  {
  }

  impl< E, const ROWS : usize, const COLS : usize, Descriptor : mat::Descriptor > Indexable
  for Mat< ROWS, COLS, E, Descriptor >
  where
    E : MatEl,
  {
    type Index = Ix2;

    #[ inline( always ) ]
    fn dim( &self ) -> Self::Index
    {
      Ix2( ROWS, COLS )
    }

  }

  impl< E, const ROWS : usize, const COLS : usize, Descriptor : mat::Descriptor > Mat< ROWS, COLS, E, Descriptor >
  where
    E : MatEl,
  {

    #[ inline( always ) ]
    pub fn dim( &self ) -> < Self as Indexable >::Index
    {
      < Self as Indexable >::dim( self )
    }

  }

  pub type Mat2< E > = Mat< 2, 2, E, DescriptorOrderColumnMajor >;
  pub type Mat3< E > = Mat< 3, 3, E, DescriptorOrderColumnMajor >;
  pub type Mat4< E > = Mat< 4, 4, E, DescriptorOrderColumnMajor >;
}

mod access_common;
mod access_mirror;
mod access_row_major;
mod access_column_major;

crate::mod_interface!
{

  layer fns;
  layer general;

  own use
  {

    Order,
    OrderRowMajor,
    OrderColumnMajor,
    Descriptor,
    DescriptorOrderRowMajor,
    DescriptorOrderColumnMajor,
    DescriptorDefault,

  };

  exposed use
  {

    MatEl,
    Mat,
    Mat2,
    Mat3,
    Mat4
  };

}
