use peroxide::fuga::*;

fn main() -> Result<(), Box<dyn Error>> {
    let basic_ode_solver = BasicODESolver::new(RK4);
    let (_, y_vec) = basic_ode_solver.solve(
        &Lorenz,
        (0f64, 100f64),
        1e-2,
    )?;
    let y_mat = py_matrix(y_vec);
    let y0 = y_mat.col(0);
    let y2 = y_mat.col(2);

    #[cfg(feature = "plot")]
    {
        let mut plt = Plot2D::new();
        plt
            .set_domain(y0)
            .insert_image(y2)
            .set_xlabel(r"$y_0$")
            .set_ylabel(r"$y_2$")
            .set_style(PlotStyle::Nature)
            .tight_layout()
            .set_dpi(600)
            .set_path("example_data/lorenz_rk4.png")
            .savefig()?;
    }

    Ok(())
}

struct Lorenz;

impl ODEProblem for Lorenz {
    fn initial_conditions(&self) -> Vec<f64> {
        vec![10f64, 1f64, 1f64]
    }

    fn rhs(&self, t: f64, y: &[f64], dy: &mut [f64]) -> Result<(), ODEError> {
        dy[0] = 10f64 * (y[1] - y[0]);
        dy[1] = 28f64 * y[0] - y[1] - y[0] * y[2];
        dy[2] = -8f64 / 3f64 * y[2] + y[0] * y[1];
        Ok(())
    }
}
