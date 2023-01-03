// Returns string for Typescript Export Default React Pure Function Component
pub fn return_tsdrpfc(name: &String) -> String {
    return format!(
        "import * as React from 'react';

export interface I{}Props {{
}}
        
export default function {} (props: I{}Props) {{
    return (
        <div>
              
        </div>
    );
}}",
        name, name, name
    );
}

// Returns string for React Arrow Function Export Component
pub fn return_rafce(name: &String) -> String {
    return format!(
        "import React from 'react'

const {} = () => {{
    return (
        <div>{}</div>
    )
}}
        
export default {}",
        name, name, name
    );
}

// Returns string for React Functional Export Component
pub fn return_rfce(name: &String) -> String {
    return format!(
        "import React from 'react'

function {}() {{
    return (
    <div>{}</div>
    )
}}
        
export default {}",
        name, name, name
    );
}

// Returns string for React Class Export Component
pub fn return_rce(name: &String) -> String {
    return format!(
        "import React, {{ Component }} from 'react'

export class {} extends Component {{
    render() {{
    return (
        <div>{}</div>
    )
    }}
}}
        
export default {}",
        name, name, name
    );
}

// Returns string for React Class Component with Redux
pub fn return_rcredux(name: &String) -> String {
    return format!(
        "import React, {{ Component }} from 'react'
import {{ connect }} from 'react-redux'
        
export class {} extends Component {{
    render() {{
        return (
            <div>{}</div>
        )
    }}
}}
        
const mapStateToProps = (state) => ({{}})
        
const mapDispatchToProps = {{}}
        
export default connect(mapStateToProps, mapDispatchToProps)({})",
        name, name, name
    );
}

// Returns string for React Functional Export component with Redux
pub fn return_rfceredux(name: &String) -> String {
    return format!(
        "import React from 'react'
import {{ connect }} from 'react-redux'
        
export const {} = (props) => {{
    return (
        <div>{}</div>
    )
}}
        
const mapStateToProps = (state) => ({{}})
        
const mapDispatchToProps = {{}}
        
export default connect(mapStateToProps, mapDispatchToProps)({})",
        name, name, name
    );
}

// Returns string for Typescript React Class component with Redux
pub fn return_tsrcredux(name: &String) -> String {
    return format!(
        "import {{ connect }} from 'react-redux'
import React, {{ Component }} from 'react'
        
type Props = {{}}
        
type State = {{}}
        
export class {} extends Component<Props, State> {{
    state = {{}}
        
    render() {{
        return (
            <div>{}</div>
        )
    }}
}}
        
const mapStateToProps = (state) => ({{}})
        
const mapDispatchToProps = {{}}
        
export default connect(mapStateToProps, mapDispatchToProps)({})",
        name, name, name
    );
}
