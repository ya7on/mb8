use crate::diagnostics::{Diagnostic, DiagnosticResult, SourceFile, SourceId};

pub(crate) trait AssemblerPass {
    type Input;
    type Output;

    fn run(&mut self, input: Self::Input, context: &mut PassContext<'_>) -> Option<Self::Output>;
}

impl<T> DiagnosticResult<T> {
    pub(crate) fn then<P>(mut self, mut pass: P) -> DiagnosticResult<P::Output>
    where
        P: AssemblerPass<Input = T>,
    {
        let Some(input) = self.result.take().filter(|_| self.ok) else {
            return DiagnosticResult {
                result: None,
                diagnostics: self.diagnostics,
                ok: self.ok,
                sources: self.sources,
            };
        };

        let (result, ok) = {
            let mut context = PassContext::new(&mut self.diagnostics, &mut self.sources, self.ok);
            let result = pass.run(input, &mut context);
            (result, context.ok)
        };
        self.ok = ok;

        DiagnosticResult {
            result,
            diagnostics: self.diagnostics,
            ok: self.ok,
            sources: self.sources,
        }
    }
}

pub(crate) struct PassContext<'a> {
    diagnostics: &'a mut Vec<Diagnostic>,
    sources: &'a mut Vec<SourceFile>,
    ok: bool,
}

impl<'a> PassContext<'a> {
    pub(crate) const fn new(
        diagnostics: &'a mut Vec<Diagnostic>,
        sources: &'a mut Vec<SourceFile>,
        ok: bool,
    ) -> Self {
        Self {
            diagnostics,
            sources,
            ok,
        }
    }

    pub(crate) fn emit(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub(crate) fn emit_fatal(&mut self, diagnostic: Diagnostic) {
        self.emit(diagnostic);
        self.ok = false;
    }

    pub(crate) fn source(&self, id: SourceId) -> Option<&SourceFile> {
        self.sources.get(id)
    }

    pub(crate) fn add_source(&mut self, name: String, source: String) -> SourceId {
        let id = self.sources.len();
        self.sources.push(SourceFile { id, name, source });
        id
    }
}
