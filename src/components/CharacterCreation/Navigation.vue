<template>
        <button id="back" @click="$emit('back')" >Back</button>
        <button id="next" @click="$emit('next')">Next</button>
</template>

<style scoped>
#back
{
    position: absolute;
    bottom: 5vh;
    left: 5vw;
}

#next
{
    position: absolute;
    bottom: 5vh;
    right: 5vw;
}
</style>
