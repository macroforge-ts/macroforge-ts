interface Props {
    items: string[];
    selected?: string;
}

let { items, selected = '' }: Props = $props();

let filteredItems = $derived(
    items.filter(item => item.includes(selected))
);

let count = $state(0);
let message = $derived.by(() => {
    if (count === 0) return 'No items';
    return `${count} items`;
});

function handleClick() {
    count++;
}

$effect(() => {
    console.log('Count changed:', count);
});
