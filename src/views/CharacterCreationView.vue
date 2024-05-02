<template>
    <div class="container">
        <NameField class="field" ref="name" v-if="current == 'name'"/>
        <RaceField class="field" ref="race" v-if="current == 'race'" :races='[{name: "dwarf", info: "rock and stone"}, {name: "elf", info: "leaf lovers"}]'/>
        <ClassField class="field" ref="class" v-if="current == 'class'"/>
        <ItemField class="field" ref="item" v-if="current == 'item'"/>
        <Navigation @next="navigate(true)" @back="navigate(false)"/>
    </div>
</template>

<style scoped>
.field
{
    margin: 1em;
}

input, select 
{
    margin: inherit;
}
</style>

<script>
import NameField from "../components/CharacterCreation/Name.vue";
import RaceField from "../components/CharacterCreation/RaceSelect.vue"
import ClassField from "../components/CharacterCreation/ClassSelect.vue"
import ItemField from "../components/CharacterCreation/ItemSelect.vue"
import Navigation from "../components/CharacterCreation/Navigation.vue";
export default
{
    components: {NameField, RaceField, ClassField, ItemField, Navigation},
    data() {
        return {
            current: "name",
            steps: ["name", "race", "class", "item"]
        }
    },
    methods:
    {
        navigate(direction)
        {
            console.log(direction);
            console.log("?")
            const index = this.steps.indexOf(this.current);
            if ((index === 0 && !direction) || (index === this.steps.length - 1 && direction)) {
                this.back(direction);
                return;
            } else 
            {
                this.current = this.steps[index + (direction? 1 : -1)];
                console.log(index);
                console.log(this.current);
            }
        },
        back(direction)
        {
            console.log("going back")
            if (direction)
            {
                //TODO: open the character menu
            } else
            {
                console.log("back to main menu")
                this.$router.push("/");
            }
        }
    }
}
</script>