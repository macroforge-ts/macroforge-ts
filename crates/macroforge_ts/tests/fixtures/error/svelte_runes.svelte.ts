let count = $state(0);
let double = $derived(count * 2);

function increment() {
    count++;
}
