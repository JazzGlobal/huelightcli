use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Light related models
pub type LightId = u32;

#[derive(Debug, Deserialize)]
pub struct LightResponse(pub HashMap<LightId, Light>);

#[derive(Debug, Deserialize, PartialEq)]
pub struct Light {
    pub state: LightState,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct LightState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    #[serde(rename = "bri", skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<u16>,
    #[serde(rename = "sat", skip_serializing_if = "Option::is_none")]
    pub saturation: Option<u8>,
}

impl LightState {
    pub fn with_on(mut self, on: bool) -> Self {
        self.on = Some(on);
        self
    }

    pub fn with_brightness(mut self, brightness: u16) -> Self {
        self.brightness = Some(brightness);
        self
    }

    pub fn with_hue(mut self, hue: u16) -> Self {
        self.hue = Some(hue);
        self
    }

    pub fn with_saturation(mut self, saturation: u8) -> Self {
        self.saturation = Some(saturation);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::models::light::LightState;

    #[test]
    pub fn light_state_serialization_omits_on_when_none() {
        // Arrange
        let light_state = LightState::default()
            .with_brightness(10)
            .with_hue(11)
            .with_saturation(12);

        let expected = serde_json::json!({
            "bri": 10,
            "hue": 11,
            "sat": 12
        });

        // Act
        let serialized = serde_json::to_string(&light_state).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&serialized).unwrap();

        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn light_state_serialization_omits_bri_when_none() {
        // Arrange
        let light_state = LightState::default()
            .with_on(true)
            .with_hue(11)
            .with_saturation(12);

        let expected = serde_json::json!({
            "on": true,
            "hue": 11,
            "sat": 12
        });

        // Act
        let serialized = serde_json::to_string(&light_state).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&serialized).unwrap();

        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn light_state_serialization_omits_hue_when_none() {
        // Arrange
        let light_state = LightState::default()
            .with_on(true)
            .with_brightness(11)
            .with_saturation(12);

        let expected = serde_json::json!({
            "on": true,
            "bri": 11,
            "sat": 12
        });

        // Act
        let serialized = serde_json::to_string(&light_state).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&serialized).unwrap();

        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn light_state_serialization_omits_sat_when_none() {
        // Arrange
        let light_state = LightState::default()
            .with_on(true)
            .with_hue(11)
            .with_brightness(12);

        let expected = serde_json::json!({
            "on": true,
            "hue": 11,
            "bri": 12
        });

        // Act
        let serialized = serde_json::to_string(&light_state).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&serialized).unwrap();

        // Assert
        assert_eq!(expected, actual);
    }
}
