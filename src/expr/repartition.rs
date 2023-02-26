// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use datafusion_expr::logical_plan::Repartition;
use pyo3::prelude::*;
use std::fmt::{self, Display, Formatter};

use crate::common::df_schema::PyDFSchema;
use crate::expr::logical_node::LogicalNode;
use crate::sql::logical::PyLogicalPlan;

#[pyclass(name = "Repartition", module = "datafusion.expr", subclass)]
#[derive(Clone)]
pub struct PyRepartition {
    repartition: Repartition,
}

impl From<Repartition> for PyRepartition {
    fn from(repartition: Repartition) -> PyRepartition {
        PyRepartition { repartition }
    }
}

impl From<PyRepartition> for Repartition {
    fn from(repartition: PyRepartition) -> Self {
        repartition.repartition
    }
}

impl Display for PyRepartition {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Repartition
            \nInput: {}
            \nPartitioning Scheme: {:?}",
            &self.repartition.input, &self.repartition.partitioning_scheme,
        )
    }
}

#[pymethods]
impl PyRepartition {
    /// Retrieves the input `LogicalPlan` to this `Repartition` node
    fn input(&self) -> PyResult<Vec<PyLogicalPlan>> {
        Ok(Self::inputs(self))
    }

    /// Retrieves the partitioning scheme for this `Repartition`
    fn partitioning_scheme(&self) -> PyPartitioning {
        self.repartition.partitioning_scheme.clone()
    }

    /// Resulting Schema for this `Repartition` node instance
    fn schema(&self) -> PyResult<PyDFSchema> {
        Ok(self.repartition.input.schema().as_ref().clone().into())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Repartition({})", self))
    }

    fn __name__(&self) -> PyResult<String> {
        Ok("Repartition".to_string())
    }
}

impl LogicalNode for PyRepartition {
    fn inputs(&self) -> Vec<PyLogicalPlan> {
        vec![PyLogicalPlan::from((*self.repartition.input).clone())]
    }
}
