/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub struct MPCView {
    input_columns: Vec<Box<dyn MPCMetric>>,
    metrics: Vec<dyn MPCMetric>,
    grouping_sets: Vec<Vec<dyn MPCMetric>>,
}

impl MPCView {
    pub fn new(
        input_columns: Vec<Box<dyn MPCMetric>>,
        metrics: Vec<dyn MPCMetric>,
        grouping_sets: Vec<Vec<dyn MPCMetric>>,
    ) -> Self {
        Self {
            input_columns,
            metrics,
            grouping_sets,
        }
    }
}