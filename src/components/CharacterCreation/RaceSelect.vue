<template>
    <div class="container">
        <Race v-for="race in races" :race="race" @select="selectRace"/>
    </div>

</template>

<script>
import Race from './Race.vue';
export default
{
    
    props:["races"],
    components: {Race},
    methods: {
        selectRace(race)
        {
            console.log("selected " + race.name)
        }
    }
}
</script>