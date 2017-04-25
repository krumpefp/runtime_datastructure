/*
    The library provides a simple datastructure to access geolocated labels with an additional
    elimination time t and a label size factor. The library provides method to query a set of
    such labels with a bounding box and a minimum elimination time.

    Copyright (C) {2017}  {Filip Krumpe <filip.krumpe@fmi.uni-stuttgart.de}

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

///
/// A module providing a simple 2 dimensional bounding box class.
///
/// A bounding box is defined by a min and max value for each coodinate.
/// A point in included in the bounding box, if its coordinate are >= min and <= max for the
/// corresponding min and max value.
///
/// The module provides several functions to check if labels (see the
/// [Label module](label/index.html)) are within a bounding box.
///
/// It further provides methods to adapt a given bounding box to contain a specific label of an
/// other bounding box.
///
pub mod bbox;

///
/// A module providing a simple label class.
///
/// A label is located at a 2 position in 2D defined by it's x and y coordinate.
/// It has an elimination time assigned (m_t). It further has an unique osm id, a priority and its
/// label string. The label factor lbl_fac defines the label size with respect to other labels.
///
/// The module provides several functions access the internal data and to compare two labels with
/// respect to one of their coordinates or t.
///
pub mod label;
