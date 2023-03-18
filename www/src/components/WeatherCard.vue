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
  line-height: 1.6;
	margin: 1rem 0;
	border: 1px solid rgba(var(--accent), 25%);
	background-color: white;
	padding: 1rem;
	border-radius: 0.4rem;
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