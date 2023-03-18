<template>
  <header>
    <nav>
      <a href="/">Home</a>
      <a href="/projects">Meteo</a>
    </nav>
  </header>
</template>
<style>
header {
  background-color: hsl(282, 42%, 53%);
  padding: 1rem;
  text-align: center;
  min-height: 5rem;
}
nav {
  display: flex;
  align-items: center;
  max-width: 20rem;
}
nav a {
  color: white;
  text-decoration: none;
  padding: 0.5rem;
}
nav a:hover {
  background-color: hsl(282, 42%, 43%);
  border-radius: 0.25rem;
}

</style>
