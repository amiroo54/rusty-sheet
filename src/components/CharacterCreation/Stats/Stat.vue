<template>
    <div class="contaner">
        <h2>
            {{ StatName }}
        </h2>
        <input type="text" id="stat" v-model="stat">

        <p id="modifier">Modifier: {{ Math.floor((stat - 10 ) / 2) }}</p>
    </div>
</template>

<script>

export default
{
    props: ['StatName'],
    data()
    {
        return {
            stat: 10
        }
    },
}
</script>

<style scoped>
.contaner
{
    flex-direction: row;
    width: 40vw
}
</style>