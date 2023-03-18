export interface DailyUnits {
  time: string;
  weathercode: string;
  temperature_2m_max: string;
  temperature_2m_min: string;
  temperature_2m_mean: string;
  apparent_temperature_max: string;
  apparent_temperature_min: string;
  apparent_temperature_mean: string;
  sunrise: string;
  sunset: string;
  shortwave_radiation_sum: string;
  precipitation_sum: string;
  rain_sum: string;
  snowfall_sum: string;
  precipitation_hours: string;
  windspeed_10m_max: string;
  windgusts_10m_max: string;
  winddirection_10m_dominant: string;
  et0_fao_evapotranspiration: string;
}

export interface Daily {
  time: string[];
  weathercode: number[];
  temperature_2m_max: number[];
  temperature_2m_min: number[];
  temperature_2m_mean: number[];
  apparent_temperature_max: number[];
  apparent_temperature_min: number[];
  apparent_temperature_mean: number[];
  sunrise: string[];
  sunset: string[];
  shortwave_radiation_sum: number[];
  precipitation_sum: number[];
  rain_sum: number[];
  snowfall_sum: number[];
  precipitation_hours: number[];
  windspeed_10m_max: number[];
  windgusts_10m_max: number[];
  winddirection_10m_dominant: number[];
  et0_fao_evapotranspiration: number[];
}

export interface MinMaxMean {
  time: string;
  value: number;
}

export interface StatMap {
  min: MinMaxMean;
  max: MinMaxMean;
  mean: MinMaxMean;
}

export interface Stats {
  temperature_2m_max: StatMap;
  temperature_2m_min: StatMap;
  temperature_2m_mean: StatMap;
  apparent_temperature_max: StatMap;
  apparent_temperature_min: StatMap;
  apparent_temperature_mean: StatMap;
  shortwave_radiation_sum: StatMap;
  precipitation_sum: StatMap;
  precipitation_hours: StatMap;
  windspeed_10m_max: StatMap;
  windgusts_10m_max: StatMap;
  et0_fao_evapotranspiration: StatMap;
  snowfall_sum: StatMap;
}

export interface ReturnOpenMeteo {
  latitude: number;
  longitude: number;
  generationtime_ms: number;
  utc_offset_seconds: number;
  timezone: string;
  timezone_abbreviation: string;
  elevation: number;
  daily: Daily; // Assuming you have the 'Daily' type defined elsewhere
  daily_units: DailyUnits; // Assuming you have the 'DailyUnits' type defined elsewhere
  daily_sorted: Daily; // Assuming you have the 'Daily' type defined elsewhere
  stats: Stats;
}