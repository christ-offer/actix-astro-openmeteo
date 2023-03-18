<script setup lang="ts">
</script>
<template>
  <article>
    <h2>
      <slot name="datapoint"></slot>
    </h2>
    <div>
    <section class="max">
      <h4>
        Max
      </h4>
      <p>
        <slot name="max-temp"></slot>
      </p>  
      <p>
        <slot name="max-date"></slot>
      </p>
    </section>
    <section class="min">
      <h4>
        Min
      </h4>
      <p>
        <slot name="min-temp"></slot>
      </p>  
      <p>
        <slot name="min-date"></slot>
      </p>
    </section>
    <section class="mean">
      <h4>
        Average
      </h4>
      <p>
        <slot name="mean-temp"></slot>
      </p>  
      <p>
        <slot name="mean-date"></slot>
      </p>
    </section>
    </div>
  </article>
</template>
<style scoped>
article {
  gap: 1rem;
  padding: 1rem;
  background-color: hsl(282, 42%, 53%);
  text-align: center; 
}
article div {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
}
article section h4 {
  font-size: 14px;
}
article section p {
  font-size: 14px;
}
</style>