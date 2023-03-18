<script setup lang="ts">
import { defineProps, ref, watch } from "vue";
import type {ReturnOpenMeteo} from "../types/openmeteo";
import SelectInput from "./SelectInput.vue";
import WeatherCard from "./WeatherCard.vue";

interface FilterOptions {
  label: string;
  value: string;
}

const filterOptions: FilterOptions[] = [
  { label: 'Temperature 2m Max', value: 'temperature_2m_max' },
  { label: 'Temperature 2m Min', value: 'temperature_2m_min' },
  { label: 'Temperature 2m Mean', value: 'temperature_2m_mean' },
  { label: 'Apparent Temperature Max', value: 'apparent_temperature_max' },
  { label: 'Apparent Temperature Min', value: 'apparent_temperature_min' },
  { label: 'Apparent Temperature Mean', value: 'apparent_temperature_mean' },
  { label: 'Precipitation Sum', value: 'precipitation_sum' },
  { label: 'Snowfall Sum', value: 'snowfall_sum' },
  { label: 'Shortwave Radiation Sum', value: 'shortwave_radiation_sum' },
  { label: 'Wind Speed 10m Max', value: 'windspeed_10m_max' },
  { label: 'Wind Gusts 10m Max', value: 'windgusts_10m_max' },
];
</script>
<template>
  <h1>
    OpenMeteo Island
  </h1>
  <p>
    This is a Vue island component inside a statically build Astro page.
  </p>
  <p>
    This is a page to show the data from the OpenMeteo API - The data is fetched from the API running on an Actix webserver (Rust btw) on localhost:3000/api/openmeteo where the statistics are calculated and delivered to the client.
  </p>
  <SelectInput v-model="filterOption" :options="filterOptions" />
  <SelectInput v-model="sortDirection" :options="sortOptions" />
  <input type="text" v-model="city" placeholder="City" />
  <input type="date" v-model="start_date" />
  <input type="date" v-model="end_date" />
  <button @click="fetchOpenMeteo(city, start_date, end_date)">Get Weather Statistics</button>
 
  
    
  <div v-if="data.latitude">
    <h1>Weather Statistics:</h1>
    <p>City: {{ city }}</p>
    <p>Start Date: {{ start_date }}</p>
    <p>End Date: {{ end_date }}</p>
    <p>Latitude: {{ data.latitude }}</p>
    <p>Longitude: {{ data.longitude }}</p>
    <p>Timezone: {{ data.timezone }}</p>
    <p>Timezone Abbr: {{ data.timezone_abbreviation }}</p>
    <p>Statitics:</p>
    <div class="stat-list">
    <WeatherCard v-for="stat in filterOptions">
      <template #datapoint>
        {{ stat.label }}
      </template>
      <template #max-temp>
        {{ data.stats[stat.value].max.value }} {{ data.daily_units[stat.value] }}
      </template>
      <template #max-date>
        {{ data.stats[stat.value].max.time }}
      </template>
      <template #min-temp>
        {{ data.stats[stat.value].min.value }} {{ data.daily_units[stat.value] }}
      </template>
      <template #min-date>
        {{ data.stats[stat.value].min.time }}
      </template>
      <template #mean-temp>
        <!-- A dirty inline function to round to two decimals -->
        {{ (Math.round(data.stats[stat.value].mean.value * 100) / 100).toFixed(2) }} {{ data.daily_units[stat.value] }} 
      </template>
      <template #mean-date>
        {{ data.stats[stat.value].mean.time }}
      </template>
    </WeatherCard>
  </div>
  </div>  
  
  <div
  v-if="loading"
  >
    <p>Loading...</p>
  </div>
  
  
</template>
<script lang="ts">
const data = ref({} as ReturnOpenMeteo);
let city = ref("");
let start_date = ref("");
let end_date = ref("");
let loading = ref(false);
let filterOption = ref("");
let sortDirection = ref("");



const sortOptions = [
  { label: 'None', value: 'none' },
  { label: 'Ascending', value: 'asc' },
  { label: 'Descending', value: 'desc' },
];


watch(
  () => filterOption.value,
  (newValue, oldValue) => {
    console.log("Filter method changed", newValue, oldValue);
  }
);

watch(
  () => sortDirection.value,
  (newValue, oldValue) => {
    console.log("Sorting method changed", newValue, oldValue);
  }
);

const fetchOpenMeteo = async (city: string, start_date: string, end_date: string) => {
  if (data) {
    data.value = {} as ReturnOpenMeteo;
  }
  loading.value = true
  const res = await fetch("http://localhost:8080/api/openmeteo", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      city: city,
      start_date: start_date,
      end_date: end_date,
    }),
  });
  const json = await res.json();
  loading.value = false
  data.value = json;
};


</script>
<style scoped>
.stat-list {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  gap: 1rem;
  max-width: 100%;
}

</style>
