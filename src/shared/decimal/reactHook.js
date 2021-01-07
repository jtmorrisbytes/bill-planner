import React from "react"

const LazyLoader = React.lazy((Component,props)=>{import("./web").then((decimal)=>{return <Component {...props} decimal={decimal}/>})})
 class DecimalWasmErrorBoundary extends React.Component {
    getDerivedStateFromError(error){
        return {hasError:true,error}
    }
    componentDidCatch(error,errorInfo){
        console.error(error,errorInfo)
    }
    render() {
        if(this.state.HasError) {
            return this.props?.renderError();
        }
        else return this.props.children
    }
}
export default function useDecimal(Component){
    return function DecimalWASMWrappedComponent(props){
        return <DecimalWasmErrorBoundary renderError = {props.renderError}>
            <React.Suspense>
                <LazyLoader render={Component} />
            </React.Suspense>
        </DecimalWasmErrorBoundary>
    }

}