/** @derive(Debug, Clone) */
class Stepper {
    accessor step: number = 1;
    label: string = '';
}

/** @derive(PartialEq, Default, Decode) */
class Dial {
    accessor turn = 0;
}
