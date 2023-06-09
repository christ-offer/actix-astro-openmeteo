<script setup lang="ts">
import { defineProps, ref, watch } from "vue";
import type {ReturnOpenMeteo} from "../types/openmeteo";
import SelectInput from "./SelectInput.vue";
import WeatherCard from "./WeatherCard.vue";

interface FilterOptions {
  label: unknown;
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

  <!--
  <SelectInput v-model="filterOption" :options="filterOptions" />
  <SelectInput v-model="sortDirection" :options="sortOptions" />
  -->
  <div class="input-container">
    <input type="text" v-model="city" placeholder="City" min="01-01-1960" max="01-01-2023" class="instructions"/>
    <input type="date" v-model="start_date" class="instructions"/>
    <input type="date" v-model="end_date" class="instructions"/>
    <button class="instructions" @click="fetchOpenMeteo(city, start_date, end_date)">Get Weather Statistics</button>
  </div>
  
    
  <div v-if="data.latitude">
    <div class="instructions location-info">
      <p>Location: {{ city }}</p>
      <p>Start Date: {{ start_date }}</p>
      <p>End Date: {{ end_date }}</p>
      <p>Latitude: {{ data.latitude }}</p>
      <p>Longitude: {{ data.longitude }}</p>
    </div>
    <h1 class="text-gradient">Selected stats:</h1>
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

.input-container {
  display: flex;
  place-content: center;
  gap: 1rem;
}

.input-container input {
  padding: 0.5rem !important;
  margin-inline: 0;
}

.input-container button {
  padding: 0.5rem !important;
  margin-inline: 0;
}

.text-gradient {
  font-size: 62px !important;
  text-align: center;
}

.location-info {
  max-width: 40%;
  margin-inline: auto;
}

.locaction-info p {
  text-align: center
}



</style>
