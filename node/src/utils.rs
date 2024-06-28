pub mod utils {
    #[macro_export]
    #[doc(hidden)]
    macro_rules! construct_async_run {
    	(|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
    		let runner = $cli.create_runner($cmd)?;
    		runner.async_run(|$config| {
    			let $components = new_partial(&$config)?;
    			let task_manager = $components.task_manager;
    			{ $( $code )* }.map(|v| (v, task_manager))
    		})
    	}}
    }
}
