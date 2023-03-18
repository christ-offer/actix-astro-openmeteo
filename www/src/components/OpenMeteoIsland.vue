<script setup lang="ts">
import { defineProps, ref } from "vue";
import type {ReturnOpenMeteo} from "../types/openmeteo";


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
    <ul>
      <li>
        Temperature 2m Max:
        <p>
          The highest recorded value is {{ data.stats.temperature_2m_max.max.value }} {{ data.daily_units.temperature_2m_max }} - it was recorded on {{ data.stats.temperature_2m_max.max.time }}
        </p>
      </li>
      <li>
        <p>
          The lowest recorded value is {{ data.stats.temperature_2m_max.min.value }} {{ data.daily_units.temperature_2m_min }} - it was recorded on {{ data.stats.temperature_2m_max.min.time }}
        </p>
      </li>
      <li>
        <p>
          The average value is {{ data.stats.temperature_2m_max.mean.value }} {{ data.daily_units.temperature_2m_mean }}
        </p>
      </li>
      <li>
        Temperature 2m Min:
        <p>
          The highest recorded value is {{ data.stats.temperature_2m_min.max.value }} {{ data.daily_units.temperature_2m_max }} - it was recorded on {{ data.stats.temperature_2m_min.max.time }}
        </p>
      </li>
      <li>
        <p>
          The lowest recorded value is {{ data.stats.temperature_2m_min.min.value }} {{ data.daily_units.temperature_2m_min }} - it was recorded on {{ data.stats.temperature_2m_min.min.time }}
        </p>
      </li>
      <li>
        <p>
          The average value is {{ data.stats.temperature_2m_min.mean.value }} {{ data.daily_units.temperature_2m_mean }}
        </p>
      </li>
      <li>
        Temperature 2m Mean:
        <p>
          The highest recorded value is {{ data.stats.temperature_2m_mean.max.value }} {{ data.daily_units.temperature_2m_max }} - it was recorded on {{ data.stats.temperature_2m_mean.max.time }}
        </p>
      </li>
      <li>
        <p>
          The lowest recorded value is {{ data.stats.temperature_2m_mean.min.value }} {{ data.daily_units.temperature_2m_min }} - it was recorded on {{ data.stats.temperature_2m_mean.min.time }}
        </p>
      </li>
      <li>
        <p>
          The average value is {{ data.stats.temperature_2m_mean.mean.value }} {{ data.daily_units.temperature_2m_mean }}
        </p>
      </li>
      <li>
        apparent_temperature_max:
        <p>
          The highest recorded value is {{ data.stats.apparent_temperature_max.max.value }} {{ data.daily_units.apparent_temperature_max }} - it was recorded on {{ data.stats.apparent_temperature_max.max.time }}
        </p>
      </li>
      <li>
        <p>
          The lowest recorded value is {{ data.stats.apparent_temperature_max.min.value }} {{ data.daily_units.apparent_temperature_min }} - it was recorded on {{ data.stats.apparent_temperature_max.min.time }}
        </p>
      </li>
      <li>
        <p>
          The average value is {{ data.stats.apparent_temperature_max.mean.value }} {{ data.daily_units.apparent_temperature_mean }}
        </p>
      </li>
      <li>
        apparent_temperature_min:
        <p>
          The highest recorded value is {{ data.stats.apparent_temperature_min.max.value }} {{ data.daily_units.apparent_temperature_max }} - it was recorded on {{ data.stats.apparent_temperature_min.max.time }}
        </p>
      </li>
      <li>
        <p>
          The lowest recorded value is {{ data.stats.apparent_temperature_min.min.value }} {{ data.daily_units.apparent_temperature_min }} - it was recorded on {{ data.stats.apparent_temperature_min.min.time }}
        </p>
      </li>
      <li>
        <p>
          The average value is {{ data.stats.apparent_temperature_min.mean.value }} {{ data.daily_units.apparent_temperature_mean }}
        </p>
      </li>
      <li>
        apparent_temperature_mean:
        <p>
          The highest recorded value is {{ data.stats.apparent_temperature_mean.max.value }} {{ data.daily_units.apparent_temperature_max }} - it was recorded on {{ data.stats.apparent_temperature_mean.max.time }}
        </p>
      </li>
      <li>
        <p>
          The lowest recorded value is {{ data.stats.apparent_temperature_mean.min.value }} {{ data.daily_units.apparent_temperature_min }} - it was recorded on {{ data.stats.apparent_temperature_mean.min.time }}
        </p>
      </li>
      <li>
        <p>
          The average value is {{ data.stats.apparent_temperature_mean.mean.value }} {{ data.daily_units.apparent_temperature_mean }}
        </p>
      </li>
    </ul>
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