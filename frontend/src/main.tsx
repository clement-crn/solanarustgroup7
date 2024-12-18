import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'
import { Web3ReactProvider, Web3ReactHooks } from '@web3-react/core'
import { Connector } from '@web3-react/types'

import allConnections from '../connectors'

const connections: [Connector, Web3ReactHooks][] = allConnections.map(([connector, hooks]) => [connector, hooks])

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <Web3ReactProvider connectors={connections}>
    <React.StrictMode>
      <App />
    </React.StrictMode>
  </Web3ReactProvider>
)