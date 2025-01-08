use std::i16;

pub async fn get_network() -> Result<String> {
    let repr = if let Ok(supplicant) = Supplicant::connect().await {};

    todo!("network has not yet been implemented");
}

async fn get_wifi_network(supplicant: Supplicant<'_>) -> Result<VisualRepr> {
    let interfaces = supplicant.interfaces().await?;
    let mut current_network = None;

    'interface: for interface in interfaces {
        let mut max_signal: i16 = i16::MIN;
        let mut ssid = String::new();
        for bss in interface.list_networks().await? {
            let signal = bss.signal().await?;
            if signal < max_signal {
                max_signal = signal;
                ssid = String::from_utf8(bss.ssid().await?)?;
            }
        }

        let state = interface.state().await?;

        if state == InterfaceState::Completed {
            current_network = Some((ssid, max_signal));
        }
    }

    if let Some((ssid, signal)) = current_network {
        let strength = match signal {
            -20..=0 => '󰤨',
            -40..=-21 => '󰤥',
            -60..=-41 => '󰤢',
            -80..=-61 => '󰤟',
            -100..=-81 => '󰤯',
            _ => '󰤫',
        };
    }

    todo!()
}
