import { useContext } from 'react';
import { EaControlContext } from '../components/EaControl';
import { EaRunner } from './EaRunner';
import { EaSetup } from './EaSetup';

export function Ea() {
    const { eaControl } = useContext(EaControlContext);

    return (<>{eaControl ? <EaRunner /> : <EaSetup />}</>);
}