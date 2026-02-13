import { renderMermaid, THEMES } from 'beautiful-mermaid'
import { writeFileSync, mkdirSync } from 'fs'

// Ensure assets directory exists
mkdirSync('assets', { recursive: true })

const theme = THEMES['tokyo-night']

// Container Execution Flow
const executionFlow = `
flowchart TD
    A[CLI / REPL Input] --> B{Parse Command}
    B -->|run| C[Container Runtime]
    B -->|pull| D[Image Manager]
    B -->|repl| E[Interactive Shell]
    
    C --> F[Setup Namespaces]
    F --> G[Configure Cgroups]
    G --> H[Mount Filesystems]
    H --> I[Chroot Into Rootfs]
    I --> J[Execute Process]
`

// System Components
const systemComponents = `
flowchart LR
    subgraph CLI["CLI Layer"]
        A[main.rs]
        B[repl.rs]
    end
    
    subgraph Core["Core Runtime"]
        C[run.rs]
        D[namespace.rs]
        E[cgroup.rs]
    end
    
    subgraph Storage["Image Storage"]
        F[storage.rs]
        G[docker.rs]
        H[pull.rs]
    end
    
    A --> C
    B --> C
    C --> D
    C --> E
    C --> F
    H --> G
`

// Container Lifecycle
const containerLifecycle = `
stateDiagram-v2
    [*] --> Created
    Created --> Running: start
    Running --> Stopped: exit
    Running --> Stopped: signal
    Stopped --> [*]: cleanup
`

async function generateDiagrams() {
  console.log('Generating diagrams with beautiful-mermaid...')
  
  try {
    const svg1 = await renderMermaid(executionFlow, theme)
    writeFileSync('assets/container-execution-flow.svg', svg1)
    console.log('✓ Generated container-execution-flow.svg')
    
    const svg2 = await renderMermaid(systemComponents, theme)
    writeFileSync('assets/system-components.svg', svg2)
    console.log('✓ Generated system-components.svg')
    
    const svg3 = await renderMermaid(containerLifecycle, theme)
    writeFileSync('assets/container-lifecycle.svg', svg3)
    console.log('✓ Generated container-lifecycle.svg')
    
    console.log('\nAll diagrams generated successfully!')
  } catch (error) {
    console.error('Error generating diagrams:', error)
    process.exit(1)
  }
}

generateDiagrams()
