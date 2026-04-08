interface Props {
    name: string;
    count?: number;
}

let { name, count = 0 }: Props = $props();
