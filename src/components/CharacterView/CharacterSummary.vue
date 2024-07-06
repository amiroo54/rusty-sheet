<template>
    <div class="container">
        <div class="texts">
            <h2>{{ character.name }}</h2> 
            <p> {{ character.race }} {{ character.class }} </p>
            
        </div>
        <div class="buttons">
            <button>View Details</button>
            <button>Delete</button>
        </div>
    </div>
</template>

<script>
export default
{
    props: ["character"]
}
</script>

<style scoped>
.container
{
    margin: 2em;
    flex-direction: row;
    flex-wrap: wrap;
    column-gap: 2em;
    justify-content: space-between;
    align-items: center;
    border-radius: 10px;
    background-color: #1b2525;
}
.buttons
{
    display: flex;
    flex-direction: column;
    margin-right: 3vw;
}
.texts
{
    display: flex;
    flex-direction: column;
    align-items: start;
    margin-left: 3vw;
}

</style>