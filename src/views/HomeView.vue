
<template>
    <div class="container">
      <img src="/src/assets/RustySheetLogo.svg" class="logo">
      <h1>Rusty sheet</h1> <h2>A simple DnD 5e character sheet</h2>

      <button @click="goToCreation()">Create character</button>
      <button>View characters</button>
      <button>Data</button>
    </div>
</template>

<script>


export default 
{
  name: "HomeView",
  methods: {
    goToCreation()
    {
      this.$router.push("/create");
    }
  }
}
</script>

<style scoped>

button, h1, h2
{
  margin: 12px 25%;
  color: rgb(222, 255, 242);
  
}

.logo 
{
  display: inline;
  height: 6em;
  padding: 1.5em;
  margin: auto;
  will-change: filter;
  transition: 0.25s;
}

.logo:hover {
  filter: drop-shadow(0 0 1em #14ffb1);
}

</style>