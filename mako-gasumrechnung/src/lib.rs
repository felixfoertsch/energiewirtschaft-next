/// Gas volume to energy conversion: m³ × Zustandszahl × Brennwert → kWh
/// This is a pure function used by all Gas billing and metering processes.
pub fn umrechnung_m3_to_kwh(volumen_m3: f64, zustandszahl: f64, brennwert_kwh_per_m3: f64) -> f64 {
	volumen_m3 * zustandszahl * brennwert_kwh_per_m3
}

/// SLP Gas allocation (temperature-dependent)
pub fn allokation_slp_gas(
	jahresverbrauch_kwh: f64,
	temperatur_celsius: f64,
	sigmoid_a: f64,
	sigmoid_b: f64,
	sigmoid_c: f64,
	sigmoid_d: f64,
) -> f64 {
	// Sigmoid function for temperature-dependent load profile
	let h = sigmoid_a / (1.0 + (sigmoid_b / (temperatur_celsius - sigmoid_c)).exp()) + sigmoid_d;
	jahresverbrauch_kwh * h / 365.0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn umrechnung_known_reference_value() {
		// 100 m³ × 0.9636 × 11.2 kWh/m³ ≈ 1079.232 kWh
		let result = umrechnung_m3_to_kwh(100.0, 0.9636, 11.2);
		assert!((result - 1079.232).abs() < 0.01, "expected ≈1079.23, got {result}");
	}

	#[test]
	fn umrechnung_zero_volume() {
		let result = umrechnung_m3_to_kwh(0.0, 0.9636, 11.2);
		assert_eq!(result, 0.0);
	}

	#[test]
	fn allokation_slp_gas_positive_for_typical_inputs() {
		// Typical SLP Gas sigmoid parameters
		let result = allokation_slp_gas(
			15000.0,  // 15,000 kWh/a
			5.0,      // 5 °C
			3.0,      // sigmoid_a
			-0.3,     // sigmoid_b
			40.0,     // sigmoid_c
			0.1,      // sigmoid_d
		);
		assert!(result > 0.0, "expected positive allocation, got {result}");
	}

	#[test]
	fn allokation_slp_gas_varies_with_temperature() {
		let params = (15000.0, 3.0, -0.3, 40.0, 0.1);
		let cold = allokation_slp_gas(params.0, -5.0, params.1, params.2, params.3, params.4);
		let warm = allokation_slp_gas(params.0, 20.0, params.1, params.2, params.3, params.4);
		// Cold temperature should yield higher allocation than warm
		assert!(cold != warm, "allocation should vary with temperature");
	}
}
