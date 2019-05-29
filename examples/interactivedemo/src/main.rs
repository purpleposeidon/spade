// Copyright 2017 The Spade Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*
 * This example is an interactive demo showing the features of spade's delaunay
 * triangulation and R-Tree. Press h for help.
 */

#![warn(clippy::all)]

extern crate cgmath;
extern crate rand;
extern crate spade;
#[macro_use]
extern crate glium;

mod cdt_example;
mod delaunay_example;
mod graphics;
mod rtree_example;
use cgmath::{BaseFloat, BaseNum, Point2};
use rand::distributions::range::SampleRange;
use rand::distributions::{Distribution, Range};
use rand::{SeedableRng, XorShiftRng};
use spade::SpadeNum;

enum App {
    RTreeDemo,
    DelaunayDemo,
    CdtDemo,
    Invalid,
}

fn main() {
    let args: Vec<_> = ::std::env::args().collect();
    let mut app = App::Invalid;
    match args.len() {
        1 => app = App::DelaunayDemo,
        2 => {
            let arg = args[1].to_lowercase();
            if arg == "delaunay" {
                app = App::DelaunayDemo;
            } else if arg == "rtree" {
                app = App::RTreeDemo;
            } else if arg == "cdt" {
                app = App::CdtDemo;
            } else {
                println!("Expected either \"delaunay\" or \"rtree\" as argument");
            }
        }
        other => println!("Expected one argument, found {}", other),
    }
    match app {
        App::RTreeDemo => rtree_example::run(),
        App::DelaunayDemo => delaunay_example::run(),
        App::CdtDemo => cdt_example::run(),
        App::Invalid => {}
    }
}

fn random_points_in_range<S: SpadeNum + SampleRange + BaseNum>(
    range: S,
    size: usize,
    seed: &[u8; 16],
) -> Vec<Point2<S>> {
    let mut rng = XorShiftRng::from_seed(*seed);
    let range = Range::new(-range, range);
    let mut points = Vec::with_capacity(size);
    for _ in 0..size {
        let x = range.sample(&mut rng);
        let y = range.sample(&mut rng);
        points.push(Point2::new(x, y));
    }
    points
}

fn random_points_with_seed<S: SpadeNum + BaseFloat + SampleRange>(
    size: usize,
    seed: &[u8; 16],
) -> Vec<Point2<S>> {
    random_points_in_range(S::one(), size, seed)
}
