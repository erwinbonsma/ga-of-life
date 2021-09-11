import { useContext, useReducer } from 'react';
import { EaSettingsContext, initialEaSettings, eaSettingsReducer } from '../components/EaSettings';
import { EaControlContext } from '../components/EaControl';
import { EaRunner } from './EaRunner';
import { EaSetup } from './EaSetup';

export function Ea() {
    const { eaControl } = useContext(EaControlContext);
    const [eaSettings, eaSettingsDispatch] = useReducer(eaSettingsReducer, initialEaSettings);

    return (
        <EaSettingsContext.Provider value={{ eaSettings, eaSettingsDispatch }}>
            {eaControl ? <EaRunner /> : <EaSetup />}
        </EaSettingsContext.Provider>
    );
}