class SampleApi {
    static get [Symbol.for("___CTOR_ARGS___")]() {
        return [];
    }
    static get [Symbol.for("___CTOR_NAME___")]() {
        return "SampleApi";
    }
}
/**
     * A sample Service that can be copied.
     * After it has been copied, this file should be deleted :)
     */ export default class SampleService {
    constructor(private readonly sampleApi: SampleApi){}
    sayHello(name: string) {
        return this.sampleApi.sample(name);
    }
    static get [Symbol.for("___CTOR_ARGS___")]() {
      return [
          "SampleApi"
      ];
    }
    static get [Symbol.for("___CTOR_NAME___")]() {
        return "SampleService";
    }
}
