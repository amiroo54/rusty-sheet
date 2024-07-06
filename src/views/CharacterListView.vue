<template>
    <CharacterSummary v-for="char in characters" :character="char"/>

</template>

<script>
import CharacterSummary from '../components/CharacterView/CharacterSummary.vue';
export default
{
    components: {CharacterSummary},
    data()
    {
        return {
            characters : [
                {name:"amiroof", race:"Warforged", class: "Cleric"}, 
                {name:"Mobin", race:"Mango", class:"Monk"}, 
                {name:"Winston Churchil", race:"British", class:"Prime minister"}
            ]
        }
    }
}
</script>