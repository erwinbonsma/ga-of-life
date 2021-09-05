import { useContext, useReducer } from 'react';
import { SettingsContext, initialSettings, settingsReducer } from '../components/EaSettings';
import { ControlContext } from '../components/EaControl';
import { EaRunner } from './EaRunner';
import { EaSetup } from './EaSetup';

export function Ea() {
    const { eaControl } = useContext(ControlContext);
    const [eaSettings, eaSettingsDispatch] = useReducer(settingsReducer, initialSettings);

    return (
        <SettingsContext.Provider value={{ eaSettings, eaSettingsDispatch }}>
            {eaControl ? <EaRunner /> : <EaSetup />}
        </SettingsContext.Provider>
    );
}