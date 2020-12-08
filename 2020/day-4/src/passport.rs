extern crate regex;

pub struct Passport {
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    county_id: String
}

impl Passport {
    pub fn new() -> Passport {
        Passport {
            birth_year: "".to_string(),
            issue_year: "".to_string(),
            expiration_year: "".to_string(),
            height: "".to_string(),
            hair_color: "".to_string(),
            eye_color: "".to_string(),
            passport_id: "".to_string(),
            county_id: "".to_string()
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{{byr:{}, iyr:{}, eyr:{}, hgt:{}, hcl:{}, ecl:{}, pid:{}, cid:{}}}",
            self.birth_year,
            self.issue_year,
            self.expiration_year,
            self.height,
            self.hair_color,
            self.eye_color,
            self.passport_id,
            self.county_id            
        );
    }

    pub fn set_data(&mut self, data_line: &str) {
        let data_line_parts = data_line.split(" ");
        for part in data_line_parts {
            let part_parts = part.split(":").collect::<Vec<&str>>();
            if part_parts[0]=="byr" {self.birth_year = part_parts[1].to_string()};
            if part_parts[0]=="iyr" {self.issue_year = part_parts[1].to_string()};
            if part_parts[0]=="eyr" {self.expiration_year = part_parts[1].to_string()};
            if part_parts[0]=="hgt" {self.height = part_parts[1].to_string()};
            if part_parts[0]=="hcl" {self.hair_color = part_parts[1].to_string()};
            if part_parts[0]=="ecl" {self.eye_color = part_parts[1].to_string()};
            if part_parts[0]=="pid" {self.passport_id = part_parts[1].to_string()};
            if part_parts[0]=="cid" {self.county_id = part_parts[1].to_string()};
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.birth_year == "" {return false;}
        if self.issue_year == "" {return false;}
        if self.expiration_year == "" {return false;}
        if self.height == "" {return false;}
        if self.hair_color == "" {return false;}
        if self.eye_color == "" {return false;}
        if self.passport_id == "" {return false;}
        // if self.county_id == "" {return false;}
        return true;
    }

    pub fn is_valid_2(&self) -> bool {
        // birth_year
        if self.birth_year == "" {return false;}
        if !regex::Regex::new(r"^[0-9]{4}$").unwrap().is_match(&self.birth_year) {return false;}
        if &self.birth_year.parse::<u32>().unwrap() < &1920 {return false;} 
        if &self.birth_year.parse::<u32>().unwrap() > &2002 {return false;} 
        
        // issue_year
        if self.issue_year == "" {return false;}
        if !regex::Regex::new(r"^[0-9]{4}$").unwrap().is_match(&self.issue_year) {return false;}
        if &self.issue_year.parse::<u32>().unwrap() < &2010 {return false;} 
        if &self.issue_year.parse::<u32>().unwrap() > &2020 {return false;} 
        
        // expiration_year
        if self.expiration_year == "" {return false;}
        if !regex::Regex::new(r"^[0-9]{4}$").unwrap().is_match(&self.expiration_year) {return false;}
        if &self.expiration_year.parse::<u32>().unwrap() < &2020 {return false;} 
        if &self.expiration_year.parse::<u32>().unwrap() > &2030 {return false;} 
        
        // height
        if self.height == "" {return false;}
        let mut height_valid = false;
        if regex::Regex::new(r"^[0-9]*cm$").unwrap().is_match(&self.height){
            // cm
            let height_number = &self.height
            .split("cm")
            .collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap();
            if height_number >= &150 && height_number <= &193 {height_valid = true};
        }
        if regex::Regex::new(r"^[0-9]*in$").unwrap().is_match(&self.height){
            // in
            let height_number = &self.height
            .split("in")
            .collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap();
            if height_number >= &59 && height_number <= &76 {height_valid = true};
        }
        if !height_valid {return false;}
        
        // hair color
        if self.hair_color == "" {return false;}
        if !regex::Regex::new(r"^\#[0-9a-f]{6}$").unwrap().is_match(&self.hair_color) {return false;}
        
        // eye color
        let valid_eye_colors = ["amb".to_string(), 
                                "blu".to_string(), 
                                "brn".to_string(), 
                                "gry".to_string(), 
                                "grn".to_string(), 
                                "hzl".to_string(), 
                                "oth".to_string()];
        if self.eye_color == "" {return false;}
        if !valid_eye_colors.contains(&self.eye_color) {return false;}
        
        // passport number
        if self.passport_id == "" {return false;}
        if !regex::Regex::new(r"^[0-9]{9}$").unwrap().is_match(&self.passport_id) {return false;}
    
        return true;
    }
}