<template>
  <select :value="modelValue" @input="onInput">
    <option v-for="option in options" :key="option.value" :value="option.value">
      {{ option.label }}
    </option>
  </select>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue'

interface Option {
  label: string
  value: string | number
}

const props = defineProps<{
  modelValue: string | number
  options: Option[]
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | number): void
}>()

const onInput = (event: Event) => {
  emit('update:modelValue', (event.target as HTMLSelectElement).value)
}
</script>
