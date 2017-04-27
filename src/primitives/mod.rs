///
/// A module providing a simple 2 dimensional bounding box class.
///
/// A bounding box is defined by a min and max value for each coodinate.
/// A point in included in the bounding box, if its coordinate are >= min and
/// <= max for the corresponding min and max value.
///
/// The module provides several functions to check if labels (see the
/// [Label module](label/index.html)) are within a bounding box.
///
/// It further provides methods to adapt a given bounding box to contain a
/// specific label of an other bounding box.
///
pub mod bbox;

///
/// A module providing a simple label class.
///
/// A label is located at a 2 position in 2D defined by it's x and y coordinate.
/// It has an elimination time assigned (m_t). It further has an unique osm id,
/// a priority and its label string.
///
/// The module provides several functions access the internal data and to
/// compare two labels with respect to one of their coordinates or t.
///
pub mod label;
