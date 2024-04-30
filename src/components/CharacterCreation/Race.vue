<template>
    <div class="container">

        <div class="race">
            <p class="name">{{ race.name }}</p>
            <div>
                <button @click="is_info = !is_info">Info</button>
                <button @click="$emit('select', race);">Select</button>
            </div>
        </div>
        <p v-if="is_info" class="info">{{ race.info }}</p>
    </div>
</template>

<script>

export default
{
    props: ["race"],
    data()
    {
        return {
            is_info: false
        }
    }
}
</script>

<style scoped>
.race
{
    display: flex;
    justify-content: space-between;
    
}

.name
{
    float: left;
}

.info
{
    transition-duration: .25s;
    flex-direction: row;
}
</style>