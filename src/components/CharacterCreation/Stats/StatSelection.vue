<template>
    <div class="container">
        <Stat v-for="i in ['Strength', 'Dexterity', 'Constitution', 'Charisma', 'Inteligence', 'Wisdom']" :StatName="i"/>
    </div>
</template>

<script>
import Stat from "./Stat.vue"

export default
{
    components: {Stat}
}
</script>

<style scoped>

.container
{
    flex-direction: row;
    flex-wrap: wrap;
}
</style>